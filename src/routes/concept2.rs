use actix_web::{HttpResponse, Responder, post, web::Json};
use regex::Regex;
use reqwest::header::{
    ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, CACHE_CONTROL, CONNECTION, HOST, HeaderMap,
    HeaderValue, ORIGIN, PRAGMA, REFERER, USER_AGENT,
};
use serde::{Deserialize, Serialize};

use crate::constants::{PACE_STANDARD, S3_HEADER};
use crate::libs::{process_concept2_time, process_distance_splits, process_time_splits};
use crate::types::{Concept2DataPoint, Mode, SplitResult};
use crate::utils::{format_time, parse_time};

#[derive(Debug, Deserialize)]
pub struct Concept2Request {
    pub mode: Mode,
    pub url: String,
    #[serde(rename = "targetIntervalStr")]
    pub target_interval: String,
}

async fn fetch_concept2_data(url: String) -> Vec<Concept2DataPoint> {
    let re =
        Regex::new(r"^https://log\.concept2\.com/(?:share|profile)/\d+/(?:log/)?\d+$").unwrap();

    if !re.is_match(&url) {
        panic!("URL invalid");
    }

    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0"));

    let client = reqwest::Client::new();
    let res = client.get(url).headers(headers).send().await.unwrap();
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

    let client = reqwest::Client::new();
    let res = client.get(url).headers(S3_HEADER.clone()).send().await.unwrap();

    res.json().await.unwrap()
}

#[post("/api/concept2")]
pub async fn serve_concept2(req: Json<Concept2Request>) -> impl Responder {
    let Concept2Request {
        mode,
        url,
        target_interval,
    } = req.into_inner();

    let data = fetch_concept2_data(url).await;

    match mode {
        Mode::Time => {
            let target = parse_time(&target_interval).expect("Invalid target time format");

            let mut cumulative_time = 0_f64;

            let result = process_concept2_time(data, target)
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
        },
        Mode::Distance => {
            HttpResponse::Ok().json(process_concept2_time(data, 600.0))
        }
    }
}
