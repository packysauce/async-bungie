use async_bungie::{BungieClient, Destiny2, destiny2::models::*};
use std::env;
use dotenv::dotenv;

#[async_attributes::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv().ok();
    let bungie = BungieClient::new(env::var("API_KEY")?);
    let user_info_card = bungie.search_destiny_player(MembershipType::All, "packysauce").await?;
    println!("{:#?}", user_info_card);
    Ok(())
}