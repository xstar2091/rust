use crate::config::DatabaseConfig;
use crate::generator::generator_trait::DatabaseReader;
use crate::generator::postgres_generator::postgres_reader::PostgresReader;

pub(crate) struct DatabaseReaderFactory {}

impl DatabaseReaderFactory {
    pub(crate) async fn new(config: &DatabaseConfig) -> Box<dyn DatabaseReader> {
        if config.typename() == "postgres" {
            return Box::new(PostgresReader::new(config).await)
        }
        panic!("create database reader failed")
    }
}
