use crate::api::embedding::EmbeddingModel;
use crate::api::embedding::onnx::OnnxEmbedding;
use crate::clients::{ApiServiceClient, QdrantClient, StorageClient};
use fred::prelude::ClientLike as _;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

pub(crate) const DOCUMENT_METADATA_COLLECTION: &str = "document_metadata";
pub(crate) const DOCUMENT_CHUNKS_COLLECTION: &str = "document_chunks";
const VECTOR_SIZE: u64 = 384; // BGE-small-en-v1.5

/// Shared application context holding all singleton dependencies.
///
/// Constructed once at startup and passed as `Arc<AppContext<EM>>` to actors,
/// services, and tools. Fields are immutable after initialization — no mutex
/// needed.
///
/// Generic over `EM` so tests can inject a mock embedding model without `dyn`.
/// Production code uses `AppContext<OnnxEmbedding>`.
pub(crate) struct AppContext<EM = OnnxEmbedding>
where
    EM: EmbeddingModel,
{
    pub(crate) db: DatabaseConnection,
    pub(crate) redis: Arc<fred::prelude::Client>,
    pub(crate) storage: StorageClient,
    pub(crate) qdrant: QdrantClient,
    pub(crate) embedding: Arc<EM>,
    pub(crate) api_service: ApiServiceClient,
}

impl AppContext<OnnxEmbedding> {
    pub(crate) async fn new() -> Result<Arc<Self>, AppContextError> {
        let db = Self::db().await?;
        let redis = Self::redis().await?;
        let storage = Self::storage().await?;
        let qdrant = Self::qdrant().await?;
        let embedding = Self::embedding().await?;
        let api_service = Self::api_service()?;

        Ok(Arc::new(Self {
            db,
            redis,
            storage,
            qdrant,
            embedding,
            api_service,
        }))
    }

    async fn db() -> Result<DatabaseConnection, AppContextError> {
        tracing::info!("Connecting to database...");
        let db_url = Self::get_env("DATABASE_URL")?;
        let db = sea_orm::Database::connect(&db_url).await.map_err(AppContextError::Db)?;
        tracing::info!("Database connected!");
        Ok(db)
    }

    async fn redis() -> Result<Arc<fred::prelude::Client>, AppContextError> {
        tracing::info!("Connecting to Redis...");
        let rd_url = Self::get_env("REDIS_URL")?;

        let rd_config = fred::types::config::Config::from_url(&rd_url).map_err(AppContextError::Redis)?;

        let client = fred::prelude::Client::new(rd_config, None, None, None);
        client.init().await.map_err(AppContextError::Redis)?;
        tracing::info!("Redis connected!");

        Ok(Arc::new(client))
    }

    async fn storage() -> Result<StorageClient, AppContextError> {
        let endpoint = Self::get_env("S3_ENDPOINT")?;
        let access_key = Self::get_env("S3_ACCESS_KEY")?;
        let secret_key = Self::get_env("S3_SECRET_KEY")?;
        let bucket = Self::get_env("S3_BUCKET")?;
        let region = Self::get_env("S3_REGION").unwrap_or_else(|_| "us-east-1".to_string());

        let credentials = aws_sdk_s3::config::Credentials::new(&access_key, &secret_key, None, None, "env");

        let config = aws_sdk_s3::Config::builder()
            .endpoint_url(&endpoint)
            .region(aws_sdk_s3::config::Region::new(region))
            .credentials_provider(credentials)
            .force_path_style(true)
            .behavior_version_latest()
            .build();

        let client = aws_sdk_s3::Client::from_conf(config);

        tracing::info!("S3 client connected to {endpoint}");
        Self::ensure_bucket(&client, &bucket).await?;

        let client = StorageClient::new(client, bucket);

        Ok(client)
    }

    async fn qdrant() -> Result<QdrantClient, AppContextError> {
        let url = Self::get_env("QDRANT_URL")?;

        let client = qdrant_client::Qdrant::from_url(&url).build().map_err(|e| {
            tracing::error!(error = %e, url, "failed to connect to qdrant");
            AppContextError::Qdrant(format!("Failed to connect: {e}"))
        })?;

        tracing::info!("Qdrant client connected to {url}");

        Self::ensure_collection(&client, DOCUMENT_METADATA_COLLECTION).await?;
        Self::ensure_collection(&client, DOCUMENT_CHUNKS_COLLECTION).await?;

        tracing::info!("Qdrant collections initialized");

        let client = QdrantClient::new(client);

        Ok(client)
    }

    async fn embedding() -> Result<Arc<OnnxEmbedding>, AppContextError> {
        tracing::info!("Initializing embedding model");
        let embedding = Arc::new(OnnxEmbedding::new().expect("Failed to initialize embedding model"));
        tracing::info!("Embedding model initialized");

        Ok(embedding)
    }

    fn api_service() -> Result<ApiServiceClient, AppContextError> {
        let url = Self::get_env("API_SERVICE_URL")?;
        tracing::info!("Initializing api-service client: {url}");
        let client = ApiServiceClient::new(&url).expect("Invalid API_SERVICE_URL");
        Ok(client)
    }

    async fn ensure_bucket(client: &aws_sdk_s3::Client, bucket: &str) -> Result<(), AppContextError> {
        if let Err(e) = client.head_bucket().bucket(bucket).send().await {
            let svc_err = e.into_service_error();
            if svc_err.is_not_found() {
                client
                    .create_bucket()
                    .bucket(bucket)
                    .send()
                    .await
                    .map_err(|_| AppContextError::Storage("Failed to create bucket"))?;

                tracing::info!("Created S3 bucket: {bucket}");
                Ok(())
            } else {
                tracing::error!("Failed to create S3 bucket: {bucket}");
                Err(AppContextError::Storage("Failed to check bucket"))
            }
        } else {
            Ok(())
        }
    }

    async fn ensure_collection(client: &qdrant_client::Qdrant, name: &str) -> Result<(), AppContextError> {
        let exists = client.collection_exists(name).await.map_err(|e| {
            tracing::error!(error = %e, name, "failed to check collection");
            AppContextError::Qdrant(format!("Failed to check collection {name}: {e}"))
        })?;

        if !exists {
            client
                .create_collection(
                    qdrant_client::qdrant::CreateCollectionBuilder::new(name).vectors_config(
                        qdrant_client::qdrant::VectorParamsBuilder::new(
                            VECTOR_SIZE,
                            qdrant_client::qdrant::Distance::Cosine,
                        ),
                    ),
                )
                .await
                .map_err(|e| {
                    tracing::error!(error = %e, name, "failed to create collection");
                    AppContextError::Qdrant(format!("Failed to create collection {name}: {e}"))
                })?;

            tracing::info!("Created Qdrant collection: {name}");
        }

        // Ensure payload indexes for tenant filtering
        for field in ["user_id", "organization_pid", "document_pid"] {
            client
                .create_field_index(qdrant_client::qdrant::CreateFieldIndexCollectionBuilder::new(
                    name,
                    field,
                    qdrant_client::qdrant::FieldType::Keyword,
                ))
                .await
                .map_err(|e| {
                    tracing::error!(error = %e, field, name, "failed to create index");
                    AppContextError::Qdrant(format!("Failed to create index {field} on {name}: {e}"))
                })?;
        }

        Ok(())
    }

    fn get_env(key: &'static str) -> Result<String, AppContextError> {
        std::env::var(key).map_err(|_| AppContextError::EnvarMissing(key))
    }
}

impl<EM> std::fmt::Debug for AppContext<EM>
where
    EM: EmbeddingModel,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppContext").finish()
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum AppContextError {
    #[error("db error: {0}")]
    Db(#[from] sea_orm::DbErr),
    #[error("{0} must be set")]
    EnvarMissing(&'static str),
    #[error("redis error: {0}")]
    Redis(#[from] fred::error::Error),
    #[error("storage error: {0}")]
    Storage(&'static str),
    #[error("qdrant error: {0}")]
    Qdrant(String),
}
