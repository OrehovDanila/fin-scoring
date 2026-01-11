mod server;

use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "fintech_scoring=info,tower_http=info".into()),
        )
        .with_target(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();

    server::run("[::1]:50051".parse()?).await
}
