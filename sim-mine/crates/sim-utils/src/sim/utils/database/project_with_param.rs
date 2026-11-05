use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(FromRow)]
pub struct Project {
    pub create_time: NaiveDateTime,
    pub desc: String,
    pub id: i64,
    pub name: String,
    pub update_time: NaiveDateTime,
    pub user_id: i64,
}

#[derive(FromRow)]
pub struct ProjectParam {
    pub project_id: i64,
    pub cloud_platform_env_id: i64,
    pub cloud_platform_project_id: i64,
    pub id: i64,
    pub map_id: i64,
    pub scenario_name: String,
    pub scenario_type: String,
    pub scene_id: i64,
    pub weather: String,
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct ProjectWithParam {
    pub user_id: i64,
    pub map_id: Option<i64>,
    pub project_id: i64,
    pub param_id: Option<i64>,
    pub scene_id: Option<i64>,
    pub cloud_platform_env_id: Option<i64>,
    pub cloud_platform_project_id: Option<i64>,
    pub name: String,
    pub desc: String,
    pub weather: Option<String>,
    pub scenario_name: Option<String>,
    pub scenario_type: Option<String>,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
}

pub struct ProjectWithParamManager {

}

impl ProjectWithParamManager {
    pub async fn select_all(pool: &PgPool) -> Result<Vec<ProjectWithParam>, sqlx::Error> {
        sqlx::query_as::<_, ProjectWithParam>(
            r#"SELECT
                   p.user_id,
                   pp.map_id,
                   p.id AS project_id,
                   pp.id AS param_id,
                   pp.scene_id,
                   pp.cloud_platform_env_id,
                   pp.cloud_platform_project_id,
                   p.name,
                   p."desc",
                   pp.weather,
                   pp.scenario_name,
                   pp.scenario_type,
                   p.create_time,
                   p.update_time
               FROM public.project AS p
               LEFT JOIN public.project_param AS pp ON pp.project_id = p.id
               WHERE p.is_delete = false
               ORDER BY p.id ASC"#,
        )
        .fetch_all(pool)
        .await
    }
}
