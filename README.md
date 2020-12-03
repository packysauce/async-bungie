# async-bungie [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
*A Rust crate for interacting with the Bungie.net API* - ***now with async!***

While the project is called "async-bungie", it mostly focuses on the Destiny 2 part of the API.

## Features

- Virtually no guarantees about how good this code is
- A direct 1:1 link to the Bungie.net API

## Usage
```Rust
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
```
