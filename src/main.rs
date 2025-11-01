// ErgTools - Rust-based web app & CLI application to easily calculate rowing splits and analyse performance.
// Copyright (C) 2025 Brian Chen (differental)
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, version 3.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

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
    const ADDR: &str = "0.0.0.0";
    const PORT: u16 = 3000;
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
