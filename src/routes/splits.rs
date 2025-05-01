use actix_web::{post, web::Json, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::utils::{format_time, parse_time, process_time_splits};

// pace shows splits for 500m
const PACE_STANDARD: f64 = 500_f64; 

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
    pub time: String,
    pub distance: String,
    pub pace: String,
    pub watts: String
}

#[post("/api/splits")]
pub async fn serve_calculator(req: Json<SplitRequest>) -> impl Responder {
    let SplitRequest { mode, known_interval, target_interval, split_input } = req.into_inner();

    match mode {
        Mode::Time => {
            let known = parse_time(&known_interval);
            let target = parse_time(&target_interval);

            let splits = split_input
                .split(|c| c == ',' || c == ' ')
                .filter(|s| !s.trim().is_empty())
                .map(|s| s.trim().parse::<u32>().expect("Invalid distance value"))
                .collect::<Vec<u32>>();

            let new_splits = process_time_splits(known, target, splits);

            let mut cumulative_time = 0_f64;

            let result = new_splits.iter().map(|x| {
                cumulative_time += x.0;
                let raw_pace = x.0 / (x.1 as f64);
                SplitResult {
                    time: format_time(cumulative_time),
                    distance: x.1.to_string(),
                    pace: format_time(raw_pace * PACE_STANDARD),
                    watts: format!("{:.1}", 2.80 * raw_pace.powi(-3))
                }
            }).collect::<Vec<SplitResult>>();

            println!("Parsed Time Mode:\n  known: {:?}\n  target: {:?}\n  new splits: {:?}", known, target, new_splits);

            return HttpResponse::Ok().json(result);
        },
        Mode::Distance => {
            // value is a distance (e.g., "500")
            // split_input is a comma-separated list of times (e.g., "0:02:00.0, 0:04:00.0")
            let known = known_interval.trim().parse::<u32>().expect("Invalid known distance");
            let target = target_interval.trim().parse::<u32>().expect("Invalid target distance");

            let splits = split_input
                .split(|c| c == ',' || c == ' ')
                .filter(|s| !s.trim().is_empty())
                .map(|s| parse_time(s))
                .collect::<Vec<f64>>();

            println!("Parsed Distance Mode:\n  known: {}\n  target: {}\n  splits: {:?}", known, target, splits);
        },
    }

    // Dummy return
    HttpResponse::Ok().json(vec![
        SplitResult {
            time: "Example time".to_string(),
            distance: "Example distance".to_string(),
            pace: "Example pace".to_string(),
            watts: "Example watts".to_string()
        },
        SplitResult {
            time: "Example time".to_string(),
            distance: "Example distance".to_string(),
            pace: "Example pace".to_string(),
            watts: "Example watts".to_string()
        },
    ])
}