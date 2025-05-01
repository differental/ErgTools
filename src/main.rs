use actix_files::Files;
use actix_web::{App, HttpServer};

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
            .service(Files::new("/", "static/"))
    })
    .bind((ADDR, PORT))?
    .run()
    .await
}
