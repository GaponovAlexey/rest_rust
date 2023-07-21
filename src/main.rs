use actix_web::{ get, web, App, HttpServer, Responder };

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
    println!("Start");
    HttpServer::new(|| App::new().service(hi).service(hello))
        .bind(("127.0.0.1", 3000))?
        .run().await
}
