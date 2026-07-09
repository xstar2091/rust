use crate::config::Config;
use crate::generator::factory::DatabaseReaderFactory;
use crate::generator::generator_trait::{DatabaseReader, HeaderGenerator};

pub struct CodeGenerator {
    // header_generator: Box<dyn HeaderGenerator>,
    database_reader: Box<dyn DatabaseReader>,
}

impl CodeGenerator {
    pub async fn new(config: &Config) -> CodeGenerator {
        CodeGenerator {
            database_reader: DatabaseReaderFactory::new(config.database()).await
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
