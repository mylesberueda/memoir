//! Playground HTTP routes — driver for UI verification, not a product surface.
//!
//! Exposes one streaming chat endpoint backed by [`memoir_core::Client`].
//! Each turn pulls context via [`memoir_core::Client::query`], writes the
//! user message via [`memoir_core::Client::remember`], and streams the
//! assistant response from the operator's configured extraction LLM via
//! rig. The assistant turn is NOT written back to memoir in this release —
//! streaming-and-buffering for the post-stream write is deferred.
//!
//! Auth runs as middleware via [`Authenticator::authenticate_credentials`],
//! sharing precedence and verification logic with the gRPC handlers.

use std::convert::Infallible;
use std::sync::Arc;

use axum::Router;
use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::middleware::{self, Next};
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{Extension, Json};
use futures::{Stream, StreamExt};
use memoir_core::llm::{ChatRole, ChatTurn, LlmRole};
use memoir_core::memory::Scope;
use serde::Deserialize;
use tonic::Code;

use crate::AppContext;
use crate::middleware::auth::CallerIdentity;

const PLAYGROUND_PREAMBLE: &str = "You are a helpful assistant with access to the user's memory. \
                                   Use the memories below as context when answering. If no memories \
                                   are relevant, answer from general knowledge.";

/// Builds the playground router with auth middleware applied.
pub(crate) fn router(ctx: Arc<AppContext>) -> Router {
    Router::new()
        .route("/playground/chat", post(chat))
        .layer(middleware::from_fn_with_state(ctx.clone(), auth_layer))
        .with_state(ctx)
}

async fn auth_layer(State(ctx): State<Arc<AppContext>>, mut request: Request, next: Next) -> Response {
    let headers = request.headers();
    let api_key = headers.get("x-api-key").and_then(|v| v.to_str().ok());
    let bearer = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|raw| raw.strip_prefix("Bearer "));

    match ctx.auth.authenticate_credentials(api_key, bearer).await {
        Ok(identity) => {
            request.extensions_mut().insert(identity);
            next.run(request).await
        }
        Err(status) => status_to_response(status),
    }
}

fn status_to_response(status: tonic::Status) -> Response {
    let http_status = match status.code() {
        Code::Unauthenticated => StatusCode::UNAUTHORIZED,
        Code::PermissionDenied => StatusCode::FORBIDDEN,
        Code::InvalidArgument => StatusCode::BAD_REQUEST,
        Code::NotFound => StatusCode::NOT_FOUND,
        Code::FailedPrecondition => StatusCode::PRECONDITION_FAILED,
        Code::Unavailable => StatusCode::SERVICE_UNAVAILABLE,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    };
    (http_status, status.message().to_string()).into_response()
}

#[derive(Debug, Deserialize)]
struct ChatRequest {
    scope: WireScope,
    message: String,
    #[serde(default)]
    history: Vec<WireTurn>,
}

#[derive(Debug, Deserialize)]
struct WireScope {
    agent_id: String,
    org_id: String,
    user_id: String,
}

#[derive(Debug, Deserialize)]
struct WireTurn {
    role: String,
    content: String,
}

async fn chat(
    State(ctx): State<Arc<AppContext>>,
    Extension(caller): Extension<CallerIdentity>,
    Json(req): Json<ChatRequest>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, Response> {
    let caller_pid = match &caller.principal {
        crate::middleware::auth::Principal::User { pid } => pid.clone(),
        crate::middleware::auth::Principal::ApiKey { pid } => pid.clone(),
    };

    if req.message.trim().is_empty() {
        return Err((StatusCode::BAD_REQUEST, "message: required").into_response());
    }
    if req.scope.agent_id.is_empty() || req.scope.org_id.is_empty() || req.scope.user_id.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "scope: agent_id, org_id, and user_id must all be non-empty",
        )
            .into_response());
    }

    let scope = Scope {
        agent_id: req.scope.agent_id,
        org_id: req.scope.org_id,
        user_id: req.scope.user_id,
    };
    let history: Vec<ChatTurn> = req
        .history
        .into_iter()
        .map(|t| ChatTurn {
            role: if t.role == "assistant" { ChatRole::Assistant } else { ChatRole::User },
            content: t.content,
        })
        .collect();

    tracing::event!(
        name: "memoir.api.playground.chat.invoked",
        tracing::Level::INFO,
        caller.pid = %caller_pid,
        scope.agent_id = %scope.agent_id,
        scope.org_id = %scope.org_id,
        scope.user_id = %scope.user_id,
        message.len = req.message.len(),
        history.turns = history.len(),
        "playground chat invoked",
    );

    // 1. Query context BEFORE writing the user turn — otherwise the query
    //    finds the just-written message in its own results.
    let context = ctx
        .memoir
        .query(&req.message, scope.clone())
        .await
        .map_err(|err| {
            tracing::error!(error.message = %err, "playground query failed");
            (StatusCode::INTERNAL_SERVER_ERROR, "query failed").into_response()
        })?;

    // 2. Write the user turn. Await (not fire-and-forget) so the enqueue
    //    is confirmed before the assistant stream starts.
    ctx.memoir
        .remember(req.message.clone(), scope.clone())
        .await
        .map_err(|err| {
            tracing::error!(error.message = %err, "playground remember failed");
            (StatusCode::INTERNAL_SERVER_ERROR, "remember failed").into_response()
        })?;

    // 3. Resolve the extraction-role LLM. 503 (not startup-fail) when missing,
    //    so the service still serves gRPC even without a playground LLM.
    let llm = ctx
        .memoir
        .llms()
        .get(LlmRole::Extraction)
        .ok_or_else(|| (StatusCode::SERVICE_UNAVAILABLE, "playground LLM not configured").into_response())?
        .clone();

    // 4. Build the preamble: system instructions + the rendered MemoryContext.
    //    MemoryContext's Display impl produces the bulleted "[date, N ago] content" form.
    let preamble = format!("{PLAYGROUND_PREAMBLE}\n\nRelevant memories:\n{context}");

    // 5. Stream from rig. Ollama is the only supported provider for v0.1.
    let token_stream = llm.stream_chat(&preamble, &req.message, history).await.map_err(|err| {
        tracing::error!(error.message = %err, "playground stream_chat failed");
        (StatusCode::INTERNAL_SERVER_ERROR, "stream failed").into_response()
    })?;

    // 6. Adapt the token stream into SSE events. Per-token errors become
    //    a single error event terminating the stream; downstream errors
    //    here become Infallible because SSE has no in-band error channel.
    let sse_stream = token_stream.map(|chunk| match chunk {
        Ok(text) => Ok(Event::default().data(text)),
        Err(err) => {
            tracing::warn!(error.message = %err, "playground token error");
            Ok(Event::default().event("error").data(err.to_string()))
        }
    });

    Ok(Sse::new(sse_stream).keep_alive(KeepAlive::default()))
}
