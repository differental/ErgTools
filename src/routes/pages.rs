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
