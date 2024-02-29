use error_chain::error_chain;
use std::io::Read;
use std::collections::HashMap;


error_chain! {
    foreign_links {
        Io(std::io::Error);
        Httprequest(reqwest::Error);
    }
}

fn main() -> Result<()> {
    let mut res =  reqwest::blocking::get("http://httpbin.org/get")?;
    //response body as a string
    let mut body = String::new();
    res.read_to_string(&mut body)?;
    
    println!("Status: {}", res.status());
    println!("body: {}", body);
    println!("headers: {:#?}", res.headers());

    // raw response
    let resp = reqwest::blocking::get("http://httpbin.org/get")?;
    println!("resp: {:#?}", resp);

    Ok(())
}