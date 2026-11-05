use axum::extract::State;
use crate::http::http_common::HttpState;

pub async fn health(State(_): State<HttpState>) -> &'static str {
    "ok"
}
