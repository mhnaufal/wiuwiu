use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest, Result};
use actix_files::NamedFile;
use std::path::PathBuf;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("YES")
}

async fn index(req: HttpRequest) -> impl Responder {
    req.query_string().to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(web::scope("/app").route("index", web::get().to(index)))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}