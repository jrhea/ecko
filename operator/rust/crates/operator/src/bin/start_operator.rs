use eigen_logging::{get_logger, init_logger, log_level::LogLevel};
use tracing::error;
use dotenv::dotenv;
use std::env;
use hello_world_avs_operator::operator::{monitor_new_tasks, register_operator, DEFAULT_IPFS_API};
use hello_world_avs_operator::ipfs::IpfsService;
use eyre::Result;

#[tokio::main]
pub async fn main() -> Result<()> {
    dotenv().ok();
    init_logger(LogLevel::Info);
    if let Err(e) = register_operator().await {
        eprintln!("Failed to register operator: {:?}", e);
    }

    // Initialize IPFS service
    let ipfs_url = env::var("IPFS_API_URL").unwrap_or_else(|_| DEFAULT_IPFS_API.to_string());
    let ipfs_service = IpfsService::new(&ipfs_url)?;

    // Start the operator tasks
    //tokio::spawn(async move {
        if let Err(e) = monitor_new_tasks().await {
            error!("Error monitoring tasks: {:?}", e);
        }
    //});

    // Keep the main thread alive
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}
