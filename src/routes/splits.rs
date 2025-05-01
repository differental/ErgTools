use actix_web::{post, web::Json, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Time,
    Distance,
}

#[derive(Debug, Deserialize)]
pub struct SplitRequest {
    pub mode: Mode,
    #[serde(rename = "knownIntervalStr")]
    pub known_interval: String,
    #[serde(rename = "targetIntervalStr")]
    pub target_interval: String,
    #[serde(rename = "splitInput")]
    pub split_input: String,
}

#[derive(Serialize)]
pub struct SplitResult {
    pub label: String,
    pub value: String,
}

#[post("/api/splits")]
pub async fn serve_calculator(req: Json<SplitRequest>) -> impl Responder {
    let SplitRequest { mode, known_interval, target_interval, split_input } = req.into_inner();

    match mode {
        Mode::Time => {
            // value is a time string (e.g., "0:20:00.0")
            // split_input is a comma-separated list of distances (e.g., "500,1000,2000")
            // Validate & parse here
            println!("Time mode: known={}, target={}, splits={}", known_interval, target_interval, split_input);
        },
        Mode::Distance => {
            // value is a distance (e.g., "500")
            // split_input is a comma-separated list of times (e.g., "0:02:00.0, 0:04:00.0")
            println!("Distance mode: known={}, target={}, splits={}", known_interval, target_interval, split_input);
        },
    }

    // Dummy return for now
    HttpResponse::Ok().json(vec![
        SplitResult {
            label: "Example Split".to_string(),
            value: "Some result".to_string(),
        }, 
        SplitResult {
            label: "Example Split 2".to_string(),
            value: "Some result 2".to_string(),
        }
    ])
}