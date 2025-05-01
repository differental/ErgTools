use actix_web::{HttpResponse, Responder, post, web::Json};
use serde::{Deserialize};

use crate::constants::PACE_STANDARD;
use crate::libs::{process_distance_splits, process_time_splits};
use crate::types::{Mode, SplitResult};
use crate::utils::{format_time, parse_time};


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

#[post("/api/splits")]
pub async fn serve_calculator(req: Json<SplitRequest>) -> impl Responder {
    let SplitRequest {
        mode,
        known_interval,
        target_interval,
        split_input,
    } = req.into_inner();

    match mode {
        Mode::Time => {
            let known = parse_time(&known_interval).expect("Invalid known time format");
            let target = parse_time(&target_interval).expect("Invalid target time format");

            let splits = split_input
                .split([',', ' ', '\n', '\r'])
                .filter(|s| !s.trim().is_empty())
                .map(|s| s.trim().parse::<u32>().expect("Invalid distance value"))
                .collect::<Vec<u32>>();

            let new_splits = process_time_splits(known, target, splits);

            let mut cumulative_time = 0_f64;

            let result = new_splits
                .iter()
                .map(|x| {
                    cumulative_time += x.0;
                    let raw_pace = x.0 / (x.1 as f64);
                    SplitResult {
                        time: format_time(cumulative_time),
                        distance: x.1.to_string(),
                        pace: format_time(raw_pace * PACE_STANDARD),
                        watts: format!("{:.1}", 2.80 * raw_pace.powi(-3)),
                    }
                })
                .collect::<Vec<SplitResult>>();

            HttpResponse::Ok().json(result)
        }
        Mode::Distance => {
            // value is a distance (e.g., "500")
            // split_input is a comma-separated list of times (e.g., "0:02:00.0, 0:04:00.0")
            let known = known_interval
                .trim()
                .parse::<u32>()
                .expect("Invalid known distance");
            let target = target_interval
                .trim()
                .parse::<u32>()
                .expect("Invalid target distance");

            let splits = split_input
                .split([',', ' ', '\n', '\r'])
                .filter(|s| !s.trim().is_empty())
                .map(|s| parse_time(s).expect("Invalid splits time format"))
                .collect::<Vec<f64>>();

            let new_splits = process_distance_splits(known, target, splits);

            let mut cumulative_distance = 0_u32;

            let result = new_splits
                .iter()
                .map(|x| {
                    cumulative_distance += x.0;
                    let raw_pace = x.1 / (x.0 as f64);
                    SplitResult {
                        time: format_time(x.1),
                        distance: cumulative_distance.to_string(),
                        pace: format_time(raw_pace * PACE_STANDARD),
                        watts: format!("{:.1}", 2.80 * raw_pace.powi(-3)),
                    }
                })
                .collect::<Vec<SplitResult>>();

            HttpResponse::Ok().json(result)
        }
    }
}
