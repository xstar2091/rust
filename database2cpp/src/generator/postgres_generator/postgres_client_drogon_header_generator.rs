use std::fs::File;
use std::io::{BufWriter, Write};
use crate::config::{Config, FormaterConfig, ModelConfig};
use crate::generator::generator_trait::DatabaseClientLibraryHeaderGenerator;
use crate::generator::indent::Indent;

pub(crate) struct PostgresClientDrogonHeaderGenerator<'a> {
    formater: &'a FormaterConfig,
    model: &'a ModelConfig,
    indent : Indent,
    library_namespace: &'static str,
}

impl<'a> PostgresClientDrogonHeaderGenerator<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            formater: &config.formater(),
            model: &config.model(),
            indent: Indent::new(),
            library_namespace: r##"namespace drogon::orm
{
class Row;
}"##,
        }
    }
}

impl<'a> DatabaseClientLibraryHeaderGenerator for PostgresClientDrogonHeaderGenerator<'a> {

    fn database_library_namespace(&self) -> &str {
        self.library_namespace
    }

    fn create_from_database_row(&self, writer: &mut BufWriter<File>) {
        writeln!(writer, "{}void FromDatabaseRow(const drogon::orm::Row& row);",
            self.indent._1
        ).expect("write header file failed");
    }
}
