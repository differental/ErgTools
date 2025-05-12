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

use actix_web::{HttpResponse, Responder, post, web::Json};
use serde::Deserialize;

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

            let mut cumulative_time = 0.0;

            let result = new_splits
                .iter()
                .map(|x| {
                    cumulative_time += x.0;
                    let raw_pace = x.0 / (x.1 as f64);
                    SplitResult {
                        time: format_time(cumulative_time, false),
                        distance: x.1.to_string(),
                        pace: format_time(raw_pace * PACE_STANDARD, false),
                        watts: format!("{:.1}", 2.80 * raw_pace.powi(-3)),
                    }
                })
                .collect::<Vec<SplitResult>>();

            HttpResponse::Ok().json(result)
        }
        Mode::Distance => {
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

            let mut cumulative_distance = 0;

            let result = new_splits
                .iter()
                .map(|x| {
                    cumulative_distance += x.0;
                    let raw_pace = x.1 / (x.0 as f64);
                    SplitResult {
                        time: format_time(x.1, false),
                        distance: cumulative_distance.to_string(),
                        pace: format_time(raw_pace * PACE_STANDARD, false),
                        watts: format!("{:.1}", 2.80 * raw_pace.powi(-3)),
                    }
                })
                .collect::<Vec<SplitResult>>();

            HttpResponse::Ok().json(result)
        }
    }
}
