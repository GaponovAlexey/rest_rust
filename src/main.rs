use actix_web::{ get, web, App, HttpServer, Responder };
use std::time::Duration;
use tokio::time::sleep;
use sqlx::{ Connection, postgres::PgConnection };

#[get("/")]
async fn hi() -> impl Responder {
    "Hello, World!"
}

#[get("/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("prepear");

    let database_url = std::env
        ::var("DATABASE_URL")
        .unwrap_or_else(|_| String::from("postgres://postgres:1234567890@db:5432"));

    while PgConnection::connect(&database_url).await.is_err() {
        sleep(Duration::from_secs(1)).await;
    }
    println!("Start");
    HttpServer::new(|| App::new().service(hi).service(hello))
        .bind(("0.0.0.0", 3000))?
        .run().await
}
