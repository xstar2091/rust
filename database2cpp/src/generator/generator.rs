use crate::config::Config;
use crate::generator::factory::Factory;
use crate::generator::generator_trait::{DatabaseReader, HeaderGenerator, JsonHeaderGenerator, JsonSourceGenerator, SourceGenerator};

pub(crate) struct Generator {
    pub(crate) database_reader: Box<dyn DatabaseReader>,
    pub(crate) header_generator: Box<dyn HeaderGenerator>,
    pub(crate) source_generator: Box<dyn SourceGenerator>,
    pub(crate) json_header_generator: Box<dyn JsonHeaderGenerator>,
    pub(crate) json_source_generator: Box<dyn JsonSourceGenerator>,
}

impl Generator {
    pub(crate) async fn new(&mut self, config: &Config) -> Generator {
        Generator {
            database_reader: Factory::create_database_reader(&config.database()).await,
            header_generator: Box::new(()),
            source_generator: Box::new(()),
            json_header_generator: Box::new(()),
            json_source_generator: Box::new(()),
        }
    }
}
