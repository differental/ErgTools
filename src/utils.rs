use reqwest::header::{
    ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, CACHE_CONTROL, CONNECTION, HOST, HeaderMap,
    HeaderValue, ORIGIN, PRAGMA, REFERER, USER_AGENT,
};
use std::error::Error;

/// Parse time in either h:mm:ss.s or mm:ss.s
pub fn parse_time(s: &str) -> Result<f64, Box<dyn Error>> {
    let parts: Vec<&str> = s.trim().split(':').collect();

    let total_seconds = match parts.len() {
        2 => {
            let minutes: f64 = parts[0].parse()?;
            let seconds: f64 = parts[1].parse()?;
            minutes * 60.0 + seconds
        }
        3 => {
            let hours: f64 = parts[0].parse()?;
            let minutes: f64 = parts[1].parse()?;
            let seconds: f64 = parts[2].parse()?;
            hours * 3600.0 + minutes * 60.0 + seconds
        }
        _ => return Err("Time format incorrect".into()),
    };

    Ok(total_seconds)
}

pub fn format_time(secs: f64, force_long: bool) -> String {
    let total_seconds = secs.floor() as u64;
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = secs % 60.0;

    if force_long || hours > 0 {
        format!("{:01}:{:02}:{:04.1}", hours, minutes, seconds)
    } else {
        format!("{:02}:{:04.1}", minutes, seconds)
    }
}

pub async fn get_concept2_request_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        ACCEPT,
        HeaderValue::from_static("application/json, text/javascript, */*; q=0.01"),
    );
    headers.insert(
        ACCEPT_ENCODING,
        HeaderValue::from_static("gzip, deflate, br, zstd"),
    );
    headers.insert(
        ACCEPT_LANGUAGE,
        HeaderValue::from_static("en-GB,en-US;q=0.9,en;q=0.8"),
    );
    headers.insert(CACHE_CONTROL, HeaderValue::from_static("no-cache"));
    headers.insert(CONNECTION, HeaderValue::from_static("keep-alive"));
    headers.insert(HOST, HeaderValue::from_static("s3.amazonaws.com"));
    headers.insert(ORIGIN, HeaderValue::from_static("https://log.concept2.com"));
    headers.insert(PRAGMA, HeaderValue::from_static("no-cache"));
    headers.insert(
        REFERER,
        HeaderValue::from_static("https://log.concept2.com/"),
    );
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/135.0.0.0 Safari/537.36"));
    headers.insert(
        "sec-ch-ua",
        HeaderValue::from_static("\"Chromium\";v=\"135\", \"Not-A.Brand\";v=\"8\""),
    );
    headers.insert("sec-ch-ua-mobile", HeaderValue::from_static("?0"));
    headers.insert("sec-ch-ua-platform", HeaderValue::from_static("\"Linux\""));
    headers.insert("sec-fetch-dest", HeaderValue::from_static("empty"));
    headers.insert("sec-fetch-mode", HeaderValue::from_static("cors"));
    headers.insert("sec-fetch-site", HeaderValue::from_static("cross-site"));
    headers.insert("sec-gpc", HeaderValue::from_static("1"));

    headers
}
