use hello_world_avs_operator::upload_file::upload_and_create_task;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    upload_and_create_task().await
}
