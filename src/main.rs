use actix_web::{web, App, HttpServer, Responder};

async fn index() -> impl Responder {
    "Hello, Actix-Web!"
}

async fn qa() -> impl Responder {
    "Hello, QA!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/", web::get().to(index)).route("/qa", web::get().to(qa))
    
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
