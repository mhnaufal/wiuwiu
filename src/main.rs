use actix_files::NamedFile;
use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use std::path::PathBuf;
use crate::cart::{Cart, ICart, Product};

mod cart;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Homepage will come soon!")
}

#[get("/cart")]
async fn show_cart() -> impl Responder {
    let apple = Product { name: String::from("apple"), quantity: 23 };

    let mut cart1 = Cart { Products: Vec::new() };

    cart1.tambah_produk(String::from("Jeruk"), 9);
    cart1.tambah_produk(String::from("Anggur"), 71);
    cart1.tampilkan_cart();

    HttpResponse::Ok().body(cart1.tampilkan_cart())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(show_cart)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
