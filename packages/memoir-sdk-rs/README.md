# polypixel-memoir-sdk

Generated gRPC client SDK for [Memoir](https://github.com/mylesberueda/memoir)'s
`memoir-service`. Use this crate when your agent talks to a running
`memoir-service` over the network.

If you'd rather run Memoir in-process — owning the Postgres connection, Qdrant
client, and embedding model yourself — use
[`polypixel-memoir-core`](https://crates.io/crates/polypixel-memoir-core)
instead. The SDK and the library are sibling surfaces of the same product; pick
the one that matches your deployment shape.

## Install

```toml
[dependencies]
polypixel-memoir-sdk = "0.1"
tonic = "0.14"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

## Quick start

```rust,no_run
use memoir_sdk::memoir::v1::memory_service_client::MemoryServiceClient;
use memoir_sdk::memoir::v1::{Scope, SearchRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = MemoryServiceClient::connect("http://localhost:5153").await?;

    let response = client
        .search(SearchRequest {
            scope: Some(Scope {
                agent_id: "my-agent".into(),
                org_id: "my-org".into(),
                user_id: "user-42".into(),
            }),
            query: "coffee preference".into(),
            limit: 5,
            metadata_filter: None,
            min_similarity: None,
        })
        .await?;

    for hit in response.into_inner().hits {
        if let Some(memory) = hit.memory {
            println!("{}", memory.content);
        }
    }
    Ok(())
}
```

Authenticated requests need an `Authorization` header on each call; attach it
via `tonic::Request::metadata_mut()` or a `tonic::service::Interceptor`. See
[memoir-service](https://github.com/mylesberueda/memoir/tree/dev/apps/memoir-service)
for the available authentication modes.

## Features

| Feature | Default | What it adds |
|---|---|---|
| `reflection` | off | Embeds `FILE_DESCRIPTOR_SET` — a ~45 KB serialized `FileDescriptorSet` covering every proto in `memoir.v1`. Only needed when *serving* gRPC reflection (e.g., to make `grpcurl list` work). Pure clients should leave it off. |

```toml
polypixel-memoir-sdk = { version = "0.1", features = ["reflection"] }
```

## What's in the crate

- Generated `tonic` clients and servers for `MemoryService`, `AdminService`,
  and `AuthService` under `memoir_sdk::memoir::v1`.
- `prost`/`pbjson` types for every message and enum in the API.
- An optional serialized proto descriptor set for gRPC reflection (see above).

The SDK is regenerated from `.proto` sources by the upstream repository's
`gen:protos` Nx target; consumers do not need `buf` or `protoc` installed.

## License

Dual-licensed under [MIT](./LICENSE-MIT) or [Apache-2.0](./LICENSE-APACHE), at
your option.
