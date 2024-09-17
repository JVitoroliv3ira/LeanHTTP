use reqwest::{header::HeaderMap, StatusCode, Version};

pub struct HttpResponse {
    pub status: StatusCode,
    pub version: Version,
    pub headers: HeaderMap,
    pub body: String,
    pub duration: u128,
}