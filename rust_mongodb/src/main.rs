use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client,
};
use std::env;
use std::error::Error;
use tokio;
#[tokio::main]
async fn main() -> Result<(), Box< dyn Error>> {
    let client_uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");
    let options = ClientOptions::parse_with_resolver_config(
        &client_uri,
        ResolverConfig::cloudflare(),
    )
    .await?;
    let client = Client::with_options(options)?;
    
    println!("Databases:");
    for name in client.list_database_names(None, None).await? {
        println!("{}", name);
    }
    Ok(())
}