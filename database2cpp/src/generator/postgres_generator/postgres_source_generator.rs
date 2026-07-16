use crate::config::{Config, FormaterConfig, ModelConfig};
use crate::generator::factory::Factory;
use crate::generator::generator_trait::{DatabaseCppTypeMapping, SourceGenerator, JsonSourceGenerator, DatabaseColumnMeta};

pub(crate) struct PostgresSourceGenerator<'a> {
    config_formater: &'a FormaterConfig,
    config_model: &'a ModelConfig,
    json_generator: Box<dyn JsonSourceGenerator + 'a>,
    type_mapping: Box<dyn DatabaseCppTypeMapping>,
}

impl<'a> PostgresSourceGenerator<'a> {
    pub(crate) fn new(config: &'a Config) -> Self {
        Self {
            config_formater: config.formater(),
            config_model: config.model(),
            json_generator: Factory::create_json_source_generator(config),
            type_mapping: Factory::create_database_to_cpp_type_mapping(config.database().typename()),
        }
    }
}

impl SourceGenerator for PostgresSourceGenerator<'_> {
    fn generate(&self, column_list: &[DatabaseColumnMeta]) {
        println!("-----------------");
        for column in column_list {
            println!("{:?}", column);
        }
    }
}
