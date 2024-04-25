mod models;
mod config;
use actix_web::{HttpServer, App, web, Responder, HttpResponse};
use crate::models::Status;
use std::io;
use dotenv::dotenv;

async fn status() -> impl Responder {
    HttpResponse::Ok()
    .json(Status {status: "OK".to_string()})
}
#[actix_rt::main]
async fn main() -> io::Result<()> {

    dotenv().ok();

    let config = crate::config::Config::from_env().unwrap();

    let addr = format!("http://{}:{}/", config.server.host, config.server.port);

    println!("hello this is starting at {}", addr);
    
    HttpServer::new(|| {
        App::new()
           .route("/", web::get().to(status))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
