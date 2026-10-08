use std::io::Write;
use std::fs::File;
use std::io::BufWriter;
use std::rc::Rc;
use crate::config::{Config, FormaterConfig, ModelConfig};
use crate::generator::generator_trait::DatabaseClientLibraryHeaderGenerator;
use crate::generator::indent::Indent;

pub(crate) struct PostgresClientPqxxHeaderGenerator<'a> {
    formater: &'a FormaterConfig,
    model: &'a ModelConfig,
    indent : Rc<Indent>,
    library_namespace: &'static str,
}

impl<'a> PostgresClientPqxxHeaderGenerator<'a> {
    pub fn new(config: &'a Config, indent: Rc<Indent>) -> Self {
        Self {
            formater: &config.formater(),
            model: &config.model(),
            indent,
            library_namespace: r##"namespace pqxx
{
class row;
}"##,
        }
    }
}

impl<'a> DatabaseClientLibraryHeaderGenerator for PostgresClientPqxxHeaderGenerator<'a> {
    fn database_library_namespace(&self) -> &str {
        self.library_namespace
    }

    fn create_from_database_row(&self, writer: &mut BufWriter<File>) {
        writeln!(writer, "{}void FromDatabaseRow(const pqxx::row& row);",
                 self.indent._1
        ).expect("write header file failed");
    }
}
