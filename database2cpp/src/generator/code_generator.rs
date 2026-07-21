use crate::config::Config;
use crate::generator::factory::Factory;
use crate::generator::generator_trait::{DatabaseReader, HeaderGenerator, SourceGenerator};

pub struct CodeGenerator<'a> {
    database_reader: Box<dyn DatabaseReader>,
    header_generator: Box<dyn HeaderGenerator + 'a>,
    source_generator: Box<dyn SourceGenerator + 'a>,
}

impl<'a> CodeGenerator<'a> {
    pub async fn new(config: &Config) -> CodeGenerator {
        CodeGenerator {
            database_reader: Factory::create_database_reader(config.database()).await,
            header_generator: Factory::create_header_generator(config),
            source_generator: Factory::create_source_generator(config),
        }
    }
    
    pub async fn generate(&mut self, config: &Config) {
        for table in config.database().table_list() {
            self.database_reader.read(config.database(), table).await;
            self.header_generator.generate(table, self.database_reader.column_meta_list());
            self.source_generator.generate(table, self.database_reader.column_meta_list());
        }
    }
}
