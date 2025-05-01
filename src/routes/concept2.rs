use actix_web::{post, web::Json, HttpResponse, Responder};
use regex::Regex;
use serde::{Deserialize, Serialize};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, CACHE_CONTROL, CONNECTION, HOST, ORIGIN, PRAGMA, REFERER, USER_AGENT};

use ergtools::{process_distance_splits, process_time_splits};

use crate::types::{Mode, Concept2DataPoint};
use crate::utils::{format_time, parse_time};

#[derive(Debug, Deserialize)]
pub struct Concept2Request {
    pub mode: Mode,
    pub url: String,
    #[serde(rename = "targetIntervalStr")]
    pub target_interval: String,
}

#[post("/api/concept2")]
pub async fn serve_concept2(req: Json<Concept2Request>) -> impl Responder {
    let Concept2Request { mode, url, target_interval } = req.into_inner();
    println!("{:?} {} {} ", mode, url, target_interval);

    let re = Regex::new(r"^https://log\.concept2\.com/(?:share|profile)/\d+/(?:log/)?\d+$").unwrap();

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
        .ok_or("user_id not found").unwrap();

    let stroke_file = stroke_file_re
        .captures(&body)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str())
        .ok_or("stroke_file not found").unwrap();

    let url = format!(
        "https://s3.amazonaws.com/data.concept2.com/strokes/{}/{}.json",
        user_id, stroke_file
    );

    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("application/json, text/javascript, */*; q=0.01"));
    headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip, deflate, br, zstd"));
    headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-GB,en-US;q=0.9,en;q=0.8"));
    headers.insert(CACHE_CONTROL, HeaderValue::from_static("no-cache"));
    headers.insert(CONNECTION, HeaderValue::from_static("keep-alive"));
    headers.insert(HOST, HeaderValue::from_static("s3.amazonaws.com"));
    headers.insert(ORIGIN, HeaderValue::from_static("https://log.concept2.com"));
    headers.insert(PRAGMA, HeaderValue::from_static("no-cache"));
    headers.insert(REFERER, HeaderValue::from_static("https://log.concept2.com/"));
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/135.0.0.0 Safari/537.36"));
    headers.insert("sec-ch-ua", HeaderValue::from_static("\"Chromium\";v=\"135\", \"Not-A.Brand\";v=\"8\""));
    headers.insert("sec-ch-ua-mobile", HeaderValue::from_static("?0"));
    headers.insert("sec-ch-ua-platform", HeaderValue::from_static("\"Linux\""));
    headers.insert("sec-fetch-dest", HeaderValue::from_static("empty"));
    headers.insert("sec-fetch-mode", HeaderValue::from_static("cors"));
    headers.insert("sec-fetch-site", HeaderValue::from_static("cross-site"));
    headers.insert("sec-gpc", HeaderValue::from_static("1"));

    let client = reqwest::Client::new();
    let res = client.get(url)
        .headers(headers)
        .send()
        .await
        .unwrap();

    let body: Vec<Concept2DataPoint> = res.json().await.unwrap();
    println!("{:?}", body);

    HttpResponse::Ok().json(vec![100])
}