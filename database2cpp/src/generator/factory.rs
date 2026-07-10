use crate::config::DatabaseConfig;
use crate::generator::generator::Generator;
use crate::generator::generator_trait::{DatabaseReader, HeaderGenerator};
use crate::generator::postgres_generator::postgres_header_generator::PostgresHeaderGenerator;
use crate::generator::postgres_generator::postgres_reader::PostgresReader;

pub(crate) struct Factory {}

impl Factory {
    pub(crate) async fn create_database_reader(config: &DatabaseConfig) -> Box<dyn DatabaseReader> {
        if config.typename() == "postgres" {
            return Box::new(PostgresReader::new(config).await)
        }
        panic!("unknown database type {}", config.typename());
    }

    pub(crate) fn create_header_generator<'a>(database_type_name: &str, generator: &'a Generator) -> Box<dyn HeaderGenerator + 'a> {
        if database_type_name == "postgres" {
            return Box::new(PostgresHeaderGenerator::new(generator))
        }
        panic!("unknown database type {}", database_type_name);
    }
}
