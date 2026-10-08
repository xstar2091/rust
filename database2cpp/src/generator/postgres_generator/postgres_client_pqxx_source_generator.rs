use std::io::Write;
use std::fs::File;
use std::io::BufWriter;
use std::rc::Rc;
use crate::config::{Config, FormaterConfig, ModelConfig};
use crate::generator::factory::Factory;
use crate::generator::generator_trait::{DatabaseClientLibrarySourceGenerator, DatabaseColumnMeta, DatabaseCppTypeMapping};
use crate::generator::indent::Indent;
use crate::generator::postgres_generator::postgres_client_drogon_source_generator::PostgresClientDrogonSourceGenerator;

pub(crate) struct PostgresClientPqxxSourceGenerator<'a> {
    formater: &'a FormaterConfig,
    model: &'a ModelConfig,
    indent : Rc<Indent>,
    type_mapping: Box<dyn DatabaseCppTypeMapping>,
    error_message: String,
    library_include: &'static str,
}

impl<'a> PostgresClientPqxxSourceGenerator<'a> {
    pub fn new(config: &'a Config, indent: Rc<Indent>) -> Self {
        Self {
            formater: &config.formater(),
            model: &config.model(),
            indent,
            type_mapping: Factory::create_database_to_cpp_type_mapping(config.database().typename()),
            error_message: String::from("write source file failed"),
            library_include: "#include <pqxx/row>",
        }
    }
}

impl<'a> DatabaseClientLibrarySourceGenerator for PostgresClientPqxxSourceGenerator<'a> {
    fn library_include(&self) -> &str {
        self.library_include
    }

    fn create_from_database_row(
        &self,
        class_name: &str,
        column_list: &[DatabaseColumnMeta],
        writer: &mut std::io::BufWriter<std::fs::File>
    ) {
        writeln!(writer, r##"void {1}::FromDatabaseRow(const pqxx::row& row)
{{
{0}if (bit_.none())
{0}{{
{2}bit_.set();
{0}}}"##, self.indent._1, class_name, self.indent._2).expect(&self.error_message);
        for column in column_list {
            let cpp_type_string = self.type_mapping.database_to_cpp_mapping(&column.data_type);
            writeln!(writer, "{0}if (has_{1}()) set_{1}(row[\"{1}\"].as<{2}>());",
                     self.indent._1, column.column_name, cpp_type_string
            ).expect(&self.error_message);
        }
        writeln!(writer, "}}\n").expect(&self.error_message);
    }
}
