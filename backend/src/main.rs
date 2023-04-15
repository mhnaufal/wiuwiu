use std::ops::DerefMut;
use actix_files::NamedFile;
use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result, delete};
use std::path::PathBuf;
use std::sync::Mutex;
use crate::cart::{Cart, ICart, Product};
use serde::{Deserialize, Serialize};

mod cart;

#[derive(Deserialize)]
struct ReqBodyPayload {
    name: String,
    quantity: u64,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Welcome to Rust Cart API!")
}

#[get("/cart")]
async fn show_cart(db: web::Data<Cart>) -> impl Responder {
    HttpResponse::Ok().json(db.products.lock().unwrap().to_vec())
}

#[post("/cart")]
async fn add_product(db: web::Data<Cart>, req_payload: web::Json<ReqBodyPayload>) -> impl Responder {
    let mut added_product = db.products.lock().unwrap();
    added_product.push(Product { name: req_payload.name.to_string(), quantity: req_payload.quantity });
    HttpResponse::Ok().json("Successfully adding a new product to cart!")
}

#[delete("/cart")]
async fn delete_product(db: web::Data<Cart>, req_payload: web::Json<ReqBodyPayload>) -> impl Responder {
    let mut deleted_product = db.products.lock().unwrap();
    *deleted_product = deleted_product.to_vec().into_iter().filter(|x| x.name != req_payload.name.to_string()).collect();
    HttpResponse::Ok().json("Successfully delete a product!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(Cart {
        products: Mutex::new(vec![])
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(hello)
            .service(show_cart)
            .service(add_product)
            .service(delete_product)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
