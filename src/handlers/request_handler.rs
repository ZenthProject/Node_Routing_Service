use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use axum::{
    response::IntoResponse,
    extract::Query,
    Json,
    http::{StatusCode, Request},
    body::Body


};


#[derive(Deserialize)]
pub struct RequestParams {
    method: Option<String>,
    data: Option<String>,
}

impl RequestParams {
    pub fn new(method: Option<String>, data: Option<String>) -> Self {
        RequestParams { method, data }
    }
}

pub async fn dynamic_handler(Query(params): Query<RequestParams>) -> impl IntoResponse {
    match params.method.as_deref() {
        Some("GET") => Json(format!("GET : {:?}", params.data)).into_response(),
        Some("POST") => Json(format!("POST : {:?}", params.data)).into_response(),
        Some("PUT") => Json(format!("PUT : {:?}", params.data)).into_response(),
        Some("DELETE") => Json(format!("DELETE : {:?}", params.data)).into_response(),
        _ => (StatusCode::BAD_REQUEST, "Invalid parameter").into_response(),
    }
}
