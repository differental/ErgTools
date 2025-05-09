use reqwest::{
    Client,
    header::{
        ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, CACHE_CONTROL, CONNECTION, HOST, HeaderMap,
        HeaderValue, ORIGIN, PRAGMA, REFERER, USER_AGENT,
    },
};
use std::sync::LazyLock;

// pace shows splits for 500m
pub const PACE_STANDARD: f64 = 500.0;

// cargo package version as specified in Cargo.toml
pub const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

// client for fetching webpage
pub static CLIENT_HTML: LazyLock<Client> = LazyLock::new(|| {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/135.0.0.0 Safari/537.36"));

    Client::builder().default_headers(headers).build().unwrap()
});

// client for fetching s3
pub static CLIENT_S3: LazyLock<Client> = LazyLock::new(|| {
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

    Client::builder().default_headers(headers).build().unwrap()
});
