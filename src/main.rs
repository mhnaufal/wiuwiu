use actix_files::NamedFile;
use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use std::path::PathBuf;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("YES")
}

struct AppState {
    app_name: String,
}

async fn index(req: HttpRequest, data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    "App name: ".to_owned() + app_name + " " + &req.query_string().to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Page"),
            }))
            .service(hello)
            .service(web::scope("/app").route("index", web::get().to(index)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
