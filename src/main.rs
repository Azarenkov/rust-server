mod domain;
mod adapters;
mod infrastructure;
mod application;

use std::error::Error;
use infrastructure::server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    server().await?;
    Ok(())
}
