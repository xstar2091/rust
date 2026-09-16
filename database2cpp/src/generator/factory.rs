use crate::config::{Config, DatabaseConfig};
use crate::generator::generator_trait::{DatabaseClientLibraryHeaderGenerator, DatabaseClientLibrarySourceGenerator, DatabaseCppTypeMapping, DatabaseReader, HeaderGenerator, JsonHeaderGenerator, JsonSourceGenerator, SourceGenerator};
use crate::generator::json_generator::jsoncpp_json_header_generator::JsoncppJsonHeaderGenerator;
use crate::generator::json_generator::jsoncpp_json_source_generator::JsoncppJsonSourceGenerator;
use crate::generator::json_generator::nlohmann_json_header_generator::NlohmannJsonHeaderGenerator;
use crate::generator::json_generator::nlohmann_json_source_generator::NlohmannJsonSourceGenerator;
use crate::generator::postgres_generator::postgres_client_drogon_header_generator::PostgresClientDrogonHeaderGenerator;
use crate::generator::postgres_generator::postgres_client_drogon_source_generator::PostgresClientDrogonSourceGenerator;
use crate::generator::postgres_generator::postgres_header_generator::PostgresHeaderGenerator;
use crate::generator::postgres_generator::postgres_reader::PostgresReader;
use crate::generator::postgres_generator::postgres_source_generator::PostgresSourceGenerator;
use crate::generator::postgres_generator::postgres_to_cpp_type_mapping::PostgresToCppTypeMapping;

pub(crate) struct Factory {}

impl Factory {
    pub(crate) async fn create_database_reader(config: &DatabaseConfig) -> Box<dyn DatabaseReader> {
        if config.typename() == "postgres" {
            return Box::new(PostgresReader::new(config).await)
        }
        panic!("unknown database type {}", config.typename());
    }

    pub(crate) fn create_header_generator<'a>(config: &'a Config) -> Box<dyn HeaderGenerator + 'a> {
        if config.database().typename() == "postgres" {
            return Box::new(PostgresHeaderGenerator::new(config))
        }
        panic!("unknown database type {}", config.database().typename());
    }
    
    pub(crate) fn create_source_generator<'a>(config: &'a Config) -> Box<dyn SourceGenerator + 'a> {
        if config.database().typename() == "postgres" {
            return Box::new(PostgresSourceGenerator::new(config))
        }
        panic!("unknown database type {}", config.database().typename());
    }

    pub(crate) fn create_database_to_cpp_type_mapping(database_type: &str) -> Box<dyn DatabaseCppTypeMapping> {
        if database_type == "postgres" {
            return Box::new(PostgresToCppTypeMapping::new())
        }
        panic!("unknown database type {}", database_type);
    }
    
    pub(crate) fn create_json_header_generator<'a>(config: &'a Config) -> Box<dyn JsonHeaderGenerator + 'a> {
        if config.json() == "nlohmann" {
            return Box::new(NlohmannJsonHeaderGenerator::new(config))
        } else if config.json() == "jsoncpp" {
            return Box::new(JsoncppJsonHeaderGenerator::new(config))
        }
        panic!("unknown json lib type {}", config.json());
    }
    
    pub(crate) fn create_json_source_generator<'a>(config: &'a Config) -> Box<dyn JsonSourceGenerator + 'a> {
        if config.json() == "nlohmann" {
            return Box::new(NlohmannJsonSourceGenerator::new(config))
        } else if config.json() == "jsoncpp" {
            return Box::new(JsoncppJsonSourceGenerator::new(config))
        }
        panic!("unknown json lib type {}", config.json());
    }
    
    pub(crate) fn create_database_client_header_generator<'a>(config: &'a Config) -> Box<dyn DatabaseClientLibraryHeaderGenerator + 'a> {
        if config.database_client_library() == "drogon" {
            return Box::new(PostgresClientDrogonHeaderGenerator::new(config))
        } else if config.database_client_library() == "pqxx" {
            todo!("pqxx")
        }
        panic!("unknown database client library {}", config.database_client_library());
    }

    pub(crate) fn create_database_client_source_generator<'a>(config: &'a Config) -> Box<dyn DatabaseClientLibrarySourceGenerator + 'a> {
        if config.database_client_library() == "drogon" {
            return Box::new(PostgresClientDrogonSourceGenerator::new(config))
        } else if config.database_client_library() == "pqxx" {
            todo!("pqxx")
        }
        panic!("unknown database client library {}", config.database_client_library());
    }
}
