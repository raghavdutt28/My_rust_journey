// use serde::Deserialize;
use error_chain::error_chain;
use reqwest::blocking::Client;
//use std::io;

error_chain!{
    foreign_links{
        IO(std::io::Error);
        Httprequest(reqwest::Error);
    }
}

// #[derive(Debug, Deserialize)]
// struct User{
//     login: String,
//     id: u32,
// }
fn main() -> Result<()> {
    let client = Client::new();
    let user = "testuser".to_string();
    let passwd: Option<String> = None;
    let res = client.get("https://httpbin.org/get").basic_auth(user, passwd).send()?;
    println!("{:#?}", res);
    Ok(())
}
