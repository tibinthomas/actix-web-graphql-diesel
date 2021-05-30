mod config;
mod models;
use crate::models::Status;

use actix_web::{web, App, HttpServer, Responder};

async fn status() -> impl Responder {
    web::HttpResponse::Ok().json(Status {
        status: "Ok".to_string(),
    })
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let config = crate::config::Config::from_env().unwrap();

    println!(
        "Starting http server at {}:{}",
        config.server.host, config.server.port
    );

    HttpServer::new(|| App::new().route("/", web::get().to(status)))
        .bind(format!("{}:{}", config.server.host, config.server.port))?
        .run()
        .await
}
