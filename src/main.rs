use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use mime_guess::from_path;
use rust_embed::RustEmbed;

mod routes;
use routes::{
    concept2::serve_concept2,
    pages::{serve_static_calculator, serve_static_concept2, serve_static_index},
    splits::serve_calculator,
};

mod utils;

mod constants;

mod types;

mod libs;

#[derive(RustEmbed)]
#[folder = "static/"]
struct Assets;

#[actix_web::get("/{_:.*}")]
async fn serve_embedded_assets(path: web::Path<String>) -> impl Responder {
    match Assets::get(path.as_str()) {
        Some(content) => HttpResponse::Ok()
            .content_type(from_path(path.as_str()).first_or_octet_stream().as_ref())
            .body(content.data.into_owned()),
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const ADDR: &str = "127.0.0.1";
    const PORT: u16 = 3002;
    println!("Running webserver at {ADDR}:{PORT}");

    HttpServer::new(|| {
        App::new()
            .service(serve_static_index)
            .service(serve_static_calculator)
            .service(serve_static_concept2)
            .service(serve_calculator)
            .service(serve_concept2)
            .service(serve_embedded_assets)
    })
    .bind((ADDR, PORT))?
    .run()
    .await
}
