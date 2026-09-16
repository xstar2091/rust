use std::io::Write;
use std::fs::File;
use std::io::BufWriter;
use crate::config::{Config, FormaterConfig, ModelConfig};
use crate::generator::generator_trait::DatabaseClientLibrarySourceGenerator;
use crate::generator::indent::Indent;

pub(crate) struct PostgresClientDrogonSourceGenerator<'a> {
    formater: &'a FormaterConfig,
    model: &'a ModelConfig,
    indent : Indent,
    error_message: String,
    library_include: &'static str,
}

impl<'a> PostgresClientDrogonSourceGenerator<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            formater: &config.formater(),
            model: &config.model(),
            indent: Indent::new(),
            error_message: String::from("write source file failed"),
            library_include: r##"#include <drogon/orm/Field.h>
#include <drogon/orm/Row.h>"##,
        }
    }
}

impl<'a> DatabaseClientLibrarySourceGenerator for PostgresClientDrogonSourceGenerator<'a> {
    fn library_include(&self) -> &str {
        self.library_include
    }

    fn create_from_database_row(&self, class_name: &str, writer: &mut BufWriter<File>) {
        todo!()
    }
}
