use bungie::{BungieClient, Destiny2};
use std::env;
use dotenv::dotenv;

#[async_attributes::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv()?;
    let bungie = BungieClient::new(env::var("API_KEY")?);
    let manifest = bungie.get_destiny_manifest().await?;
    println!("{:?}", manifest);
    Ok(())
}