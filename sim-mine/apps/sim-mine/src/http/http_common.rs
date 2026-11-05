use sqlx::PgPool;
use crate::config::Config;
use std::sync::Arc;
use serde::Serialize;

#[derive(Clone)]
pub struct HttpState {
    pub config: Arc<Config>,
    pub pg_pool: PgPool,
}

impl HttpState {
    pub async fn load(config: Arc<Config>) -> Result<Self, Box<dyn std::error::Error>> {
        let database_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            config.database().user(), config.database().password(),
            config.database().host(), config.database().port(),
            config.database().name()
        );
        let pg_pool = PgPool::connect(&database_url)
            .await
            .map_err(|err| {
                tracing::error!("connect to database failed, {}", err);
                err
            })?;
        Ok(Self{
            config,
            pg_pool,
        })
    }

    pub fn pg_pool(&self) -> &PgPool {
        &self.pg_pool
    }
}

#[derive(Serialize)]
pub struct HttpErrorBody {
    pub(crate) error_message: String,
    pub(crate) trace_id: String,
}
