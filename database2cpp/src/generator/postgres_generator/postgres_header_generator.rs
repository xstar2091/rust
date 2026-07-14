use crate::config::{Config, FormaterConfig, ModelConfig};
use crate::generator::factory::Factory;
use crate::generator::generator_trait::{DatabaseCppTypeMapping, HeaderGenerator, JsonHeaderGenerator};

pub(crate) struct PostgresHeaderGenerator<'a> {
    config_formater: &'a FormaterConfig,
    config_model: &'a ModelConfig,
    json_generator: Box<dyn JsonHeaderGenerator + 'a>,
    type_mapping: Box<dyn DatabaseCppTypeMapping>,
}

impl<'a> PostgresHeaderGenerator<'a> {
    pub(crate) fn new(config: &'a Config) -> Self {
        Self {
            config_formater: config.formater(),
            config_model: config.model(),
            json_generator: Factory::create_json_header_generator(config),
            type_mapping: Factory::create_database_to_cpp_type_mapping(config.database().typename()),
        }
    }
}

impl HeaderGenerator for PostgresHeaderGenerator<'_> {}
