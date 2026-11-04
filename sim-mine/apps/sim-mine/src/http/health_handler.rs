use axum::extract::State;
use crate::http::http_state::HttpState;

pub async fn health(State(_): State<HttpState>) -> &'static str {
    "ok"
}
