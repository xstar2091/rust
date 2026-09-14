use std::fs::File;
use std::io::{BufWriter, Write};
use crate::config::{Config, FormaterConfig, ModelConfig};
use crate::generator::generator_trait::DatabaseClientLibraryHeaderGenerator;

pub(crate) struct PostgresClientDrogonHeaderGenerator<'a> {
    formater: &'a FormaterConfig,
    model: &'a ModelConfig,
}

impl<'a> PostgresClientDrogonHeaderGenerator<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            formater: &config.formater(),
            model: &config.model(),
        }
    }
}

impl<'a> DatabaseClientLibraryHeaderGenerator for PostgresClientDrogonHeaderGenerator<'a> {
    fn create_database_library_namespace(&self, writer: &mut BufWriter<File>) {
        todo!()
    }

    fn create_from_database_row(&self, writer: &mut BufWriter<File>) {
        todo!()
    }
}
