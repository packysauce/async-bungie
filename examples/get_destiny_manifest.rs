use anyhow::Context;
use async_bungie::{BungieClient, Destiny2};
use std::env;
use dotenv::dotenv;

#[async_attributes::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv().ok();
    let bungie = BungieClient::new(env::var("API_KEY")?);
    let manifest = bungie.get_destiny_manifest().await
        .with_context(|| "failed to get destiny manifest!")?;
    println!("{:#?}", manifest);
    Ok(())
}