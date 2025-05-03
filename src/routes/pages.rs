use actix_web::{HttpResponse, get};
use askama::Template;

use crate::constants::CARGO_PKG_VERSION;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexPageTemplate<'a> {
    version: &'a str,
}

#[get("/")]
pub async fn serve_static_index() -> HttpResponse {
    let template = IndexPageTemplate {
        version: CARGO_PKG_VERSION,
    };
    let rendered = template.render().unwrap();

    HttpResponse::Ok().body(rendered)
}

#[derive(Template)]
#[template(path = "concept2.html")]
struct Concept2PageTemplate<'a> {
    version: &'a str,
}

#[get("/concept2")]
pub async fn serve_static_concept2() -> HttpResponse {
    let template = Concept2PageTemplate {
        version: CARGO_PKG_VERSION,
    };
    let rendered = template.render().unwrap();

    HttpResponse::Ok().body(rendered)
}

#[derive(Template)]
#[template(path = "calculator.html")]
struct CalculatorPageTemplate<'a> {
    version: &'a str,
}

#[get("/calculator")]
pub async fn serve_static_calculator() -> HttpResponse {
    let template = CalculatorPageTemplate {
        version: CARGO_PKG_VERSION,
    };
    let rendered = template.render().unwrap();

    HttpResponse::Ok().body(rendered)
}
