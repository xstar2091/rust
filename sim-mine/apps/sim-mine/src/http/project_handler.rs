use axum::extract::{Path, State};
use serde::{Deserialize, Serialize};
use crate::http::http_state::HttpState;

#[derive(Serialize, Deserialize)]
struct ProjectList {
    project_list: Vec<ProjectItem>
}

#[derive(Serialize, Deserialize)]
struct ProjectItem {
    cloud_platform_env_id: i64,
    cloud_platform_project_id: i64,
    create_time: String,
    desc: String,
    id: i64,
    map_id: i64,
    name: String,
    running: bool,
    scene_id: i64,
    sim_type: String,
}

pub async fn project_list_get(
    State(state): State<HttpState>,
    Path(user_id): Path<i64>,
) -> impl axum::response::IntoResponse {
    tracing::info!("getting list of project items");
    "ok"
}
