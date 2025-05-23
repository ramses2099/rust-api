use actix_web::{App, HttpResponse, HttpServer, Responder, web};

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(health_check)))
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}
