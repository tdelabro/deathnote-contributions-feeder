use anyhow::Result;
use dotenv::dotenv;
use marketplace_infrastructure::logger;

#[tokio::main]
async fn main() -> Result<()> {
	dotenv().ok();
	logger::set_default_global_logger().cancel_reset();

	marketplace_core::event_listeners_main().await?;

	Ok(())
}
