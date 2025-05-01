use actix_web::{App, HttpServer};

mod routes;
use routes::{
    concept2::serve_concept2,
    pages::{serve_static_calculator, serve_static_concept2, serve_static_index},
    splits::serve_calculator,
};

mod utils;

mod types;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(serve_static_index)
            .service(serve_static_calculator)
            .service(serve_static_concept2)
            .service(serve_calculator)
            .service(serve_concept2)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
