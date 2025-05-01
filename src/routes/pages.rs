use actix_web::{HttpResponse, get};
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexPageTemplate;

#[get("/")]
pub async fn serve_static_index() -> HttpResponse {
    let template = IndexPageTemplate {};
    let rendered = template.render().unwrap();

    HttpResponse::Ok().body(rendered)
}

#[derive(Template)]
#[template(path = "concept2.html")]
struct Concept2PageTemplate;

#[get("/concept2")]
pub async fn serve_static_concept2() -> HttpResponse {
    let template = Concept2PageTemplate {};
    let rendered = template.render().unwrap();

    HttpResponse::Ok().body(rendered)
}

#[derive(Template)]
#[template(path = "calculator.html")]
struct CalculatorPageTemplate;

#[get("/calculator")]
pub async fn serve_static_calculator() -> HttpResponse {
    let template = CalculatorPageTemplate {};
    let rendered = template.render().unwrap();

    HttpResponse::Ok().body(rendered)
}
