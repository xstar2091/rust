use crate::config::Config;
use crate::generator::factory::Factory;
use crate::generator::generator_trait::{DatabaseReader, HeaderGenerator};

pub struct CodeGenerator<'a> {
    database_reader: Box<dyn DatabaseReader>,
    header_generator: Box<dyn HeaderGenerator + 'a>,
}

impl<'a> CodeGenerator<'a> {
    pub async fn new(config: &Config) -> CodeGenerator {
        CodeGenerator {
            database_reader: Factory::create_database_reader(config.database()).await,
            header_generator: Factory::create_header_generator(config),
        }
    }
    
    pub async fn generate(&mut self, config: &Config) {
        for table in config.database().table_list() {
            self.database_reader.read(config.database(), table).await;
            for column in self.database_reader.column_meta_list() {
                println!("{:?}", column);
            }
        }
    }
}
