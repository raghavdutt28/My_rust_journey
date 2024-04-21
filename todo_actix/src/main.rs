mod models;
use actix_web::{HttpServer, App, web, Responder, HttpResponse};
use crate::models::Status;
use std::io;

async fn status() -> impl Responder {
    HttpResponse::Ok()
    .json(Status {status: "OK".to_string()})
}
#[actix_rt::main]
async fn main() -> io::Result<()> {
    println!("hello this is starting port 8080");
    HttpServer::new(|| {
        App::new()
           .route("/", web::get().to(status))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
