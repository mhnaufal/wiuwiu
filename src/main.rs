use actix_files::NamedFile;
use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result, delete};
use std::path::PathBuf;
use crate::cart::{Cart, ICart, Product};
use serde::{Deserialize, Serialize};

mod cart;

#[derive(Deserialize)]
struct Info {
    name: String,
    quantity: u64,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Homepage will come soon!")
}

#[get("/cart")]
async fn show_cart() -> impl Responder {
    let mut cart1 = Cart { products: Vec::new() };

    cart1.tambah_produk(String::from("Jeruk"), 9);
    cart1.tambah_produk(String::from("Anggur"), 71);
    cart1.tampilkan_cart();

    HttpResponse::Ok().body(cart1.tampilkan_cart())
}

#[post("/cart")]
async fn add_product(info: web::Json<Info>) -> impl Responder {
    let _apple = Product { name: String::from("apple"), quantity: 23 };

    let mut cart = Cart { products: Vec::new() };

    cart.tambah_produk(info.name.to_string(), info.quantity);

    HttpResponse::Ok().body("successfully adding a new product")
}

// #[delete("/cart")]
// async fn delete_product() -> impl Responder {}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(show_cart)
            .service(add_product)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
