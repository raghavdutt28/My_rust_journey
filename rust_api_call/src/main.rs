use serde::Deserialize;
//use reqwest::Error;
use reqwest::header::USER_AGENT;
use error_chain::error_chain;

error_chain!{
    foreign_links {
        Io(std::io::Error);
        Httprequest(reqwest::Error);
    }
}
#[derive(Debug, Deserialize)]
struct User{
    login: String,
    id: u32,
}
#[tokio::main]
async fn main() -> Result<()> {
    let req = format!("https://api.github.com/repos/{owner}/{repo}/stargazers",
    owner = "rust-lang-nursery",
    repo = "rust-cookbook",
    );
    println!{"{:?}", req};
    let client = reqwest::Client::new();
    let res = client.get(&req).header(USER_AGENT, "Rust web-api").send().await?;

    let users: Vec<User> = res.json().await?;
    println!("{:?}", users);
    Ok(())
}
