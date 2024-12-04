use eyre::Result;
use std::path::PathBuf;
use clap::Parser;
use hello_world_utils::{
    parse_hello_world_service_manager,
    helloworldservicemanager::HelloWorldServiceManager,
};
use eigen_utils::get_signer;
use eigen_logging::{get_logger, init_logger, log_level::LogLevel};
use dotenv::dotenv;

use crate::ipfs::IpfsService;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the file to upload
    #[arg(short, long)]
    file: PathBuf,

    /// IPFS API URL (optional, defaults to http://localhost:5001)
    #[arg(short, long, default_value = "http://localhost:5001")]
    ipfs_url: String,

    /// Private key for creating task (optional, will use PRIVATE_KEY env var if not provided)
    #[arg(short, long)]
    private_key: Option<String>,
}

pub async fn upload_and_create_task() -> Result<()> {
    dotenv().ok();
    let args = Args::parse();
    init_logger(LogLevel::Info);

    // Initialize IPFS service
    let ipfs_service = IpfsService::new(&args.ipfs_url)?;
    
    // Upload file to IPFS
    get_logger().info(&format!("Uploading file: {:?}", args.file), "");
    let cid = ipfs_service.pin_file(&args.file).await?;
    get_logger().info(&format!("File uploaded successfully with CID: {}", cid), "");

    // Create task with the CID
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let path = format!("{}/../../../../contracts/deployments/hello-world/31337.json", manifest_dir);
    let hello_world_contract_address = parse_hello_world_service_manager(&path)?;
    
    // Get private key from args or environment
    let private_key = args.private_key
        .or_else(|| std::env::var("PRIVATE_KEY").ok())
        .ok_or_else(|| eyre::eyre!("No private key provided"))?;
    
    let signer = get_signer(&private_key, "http://localhost:8545");
    let hello_world_contract = HelloWorldServiceManager::new(hello_world_contract_address, signer);
    
    // Create the task using the CID as the name
    get_logger().info("Creating task with CID as name...", "");
    let cid_clone = cid.clone();
    let tx = hello_world_contract
        .createNewTask(cid)
        .send()
        .await?
        .get_receipt()
        .await?;

    println!(
        "Transaction successfull with tx : {:?}",
        tx.transaction_hash
    );
    
    get_logger().info(&format!("Task created successfully with CID: {}", cid_clone), "");
    
    Ok(())
}
