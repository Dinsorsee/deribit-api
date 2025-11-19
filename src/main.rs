use anyhow::Result;
use deribit_api::auth::get_access_token;

#[tokio::main]
async fn main() -> Result<()> {
    get_access_token().await?;
    Ok(())
}
