use actix_web::{HttpResponse, Responder, post, web::Json};
use regex::Regex;
use serde::Deserialize;

use crate::constants::{CLIENT_HTML, CLIENT_S3, PACE_STANDARD};
use crate::libs::{process_concept2_distance, process_concept2_time};
use crate::types::{Concept2DataPoint, Mode, SplitResult};
use crate::utils::{format_time, parse_time};

#[derive(Debug, Deserialize)]
pub struct Concept2Request {
    pub mode: Mode,
    pub url: String,
    #[serde(rename = "targetIntervalStr")]
    pub target_interval: String,
}

async fn fetch_concept2_data(url: &str) -> Vec<Concept2DataPoint> {
    let re =
        Regex::new(r"^https://log\.concept2\.com/(?:share|profile)/\d+/(?:log/)?\d+/?$").unwrap();

    if !re.is_match(url) {
        panic!("URL invalid");
    }

    let res = CLIENT_HTML.get(url).send().await.unwrap();
    let body = res.text().await.unwrap();

    let user_id_re = Regex::new(r#"var\s+user_id\s*=\s*(\d+);"#).unwrap();
    let stroke_file_re = Regex::new(r#"var\s+stroke_file\s*=\s*"([a-f0-9]+)";"#).unwrap();

    let user_id = user_id_re
        .captures(&body)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str())
        .ok_or("user_id not found")
        .unwrap();

    let stroke_file = stroke_file_re
        .captures(&body)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str())
        .ok_or("stroke_file not found")
        .unwrap();

    let url = format!(
        "https://s3.amazonaws.com/data.concept2.com/strokes/{}/{}.json",
        user_id, stroke_file
    );

    let res = CLIENT_S3.get(url).send().await.unwrap();
    let results: Vec<Concept2DataPoint> = res.json().await.unwrap();

    results
}

#[post("/api/concept2")]
pub async fn serve_concept2(req: Json<Concept2Request>) -> impl Responder {
    let Concept2Request {
        mode,
        url,
        target_interval,
    } = req.into_inner();

    let data = fetch_concept2_data(url.trim()).await;

    match mode {
        Mode::Time => {
            let target = parse_time(&target_interval).expect("Invalid target time format");

            let mut cumulative_time = 0.0;

            let result = process_concept2_time(data, target)
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
            let target = target_interval
                .trim()
                .parse::<u32>()
                .expect("Invalid target distance");

            let mut cumulative_distance = 0;

            let result = process_concept2_distance(data, target)
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
