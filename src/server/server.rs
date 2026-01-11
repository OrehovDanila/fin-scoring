use std::time::Instant;
use tonic::{Request, Response, Status, transport::Server};
use tracing::{info, instrument};

use scoring_engine::engine::score;

pub mod pb {
    tonic::include_proto!("scoring");
}

use pb::scoring_service_server::{ScoringService, ScoringServiceServer};
use scoring_engine::model::ScoringInput;

#[derive(Default)]
pub struct ScoringSvc;

#[tonic::async_trait]
impl ScoringService for ScoringSvc {
    #[instrument(
        name = "grpc.score",
        skip(self),
        fields(
            request = ?request.get_ref()
        )
    )]
    async fn score(
        &self,
        request: Request<pb::ScoreRequest>,
    ) -> Result<Response<pb::ScoreResponse>, Status> {
        let input = request.into_inner();

        let start = Instant::now();

        // CPU-bound → blocking pool
        let result = tokio::task::spawn_blocking(move || score(ScoringInput::from(input)))
            .await
            .map_err(|_| Status::internal("scoring failed"))?;

        let elapsed = start.elapsed();

        info!(
            score = result.score,
            decision = ?result.decision,
            latency_ms = elapsed.as_millis(),
            "scoring completed"
        );

        Ok(Response::new(pb::ScoreResponse::from(result)))
    }
}

pub async fn run(addr: std::net::SocketAddr) -> anyhow::Result<()> {
    use tokio::signal;

    info!("Starting server on {:?}", addr);

    let svc = ScoringServiceServer::new(ScoringSvc::default());

    // Канал для graceful shutdown
    let shutdown_signal = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
        info!("Shutdown signal received, stopping server...");
    };

    Server::builder()
        .add_service(svc)
        .serve_with_shutdown(addr, shutdown_signal)
        .await?;

    info!("Server stopped gracefully");

    Ok(())
}
