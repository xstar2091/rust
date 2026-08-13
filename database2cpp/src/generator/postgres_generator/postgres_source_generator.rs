use std::io::Write;
use crate::config::{Config, FormaterConfig, ModelConfig};
use crate::generator::common_utils::CommonUtils;
use crate::generator::cpp_type_enum::CppType;
use crate::generator::factory::Factory;
use crate::generator::generator_trait::{DatabaseCppTypeMapping, SourceGenerator, JsonSourceGenerator, DatabaseColumnMeta};
use crate::generator::indent::Indent;

pub(crate) struct PostgresSourceGenerator<'a> {
    config_formater: &'a FormaterConfig,
    config_model: &'a ModelConfig,
    indent : Indent,
    json_generator: Box<dyn JsonSourceGenerator + 'a>,
    type_mapping: Box<dyn DatabaseCppTypeMapping>,
    row_class_name: String,
    table_class_name: String,
    error_message: String,
}

impl<'a> PostgresSourceGenerator<'a> {
    pub(crate) fn new(config: &'a Config) -> Self {
        Self {
            config_formater: config.formater(),
            config_model: config.model(),
            indent: Indent::new(),
            json_generator: Factory::create_json_source_generator(config),
            type_mapping: Factory::create_database_to_cpp_type_mapping(config.database().typename()),
            row_class_name: String::new(),
            table_class_name: String::new(),
            error_message: String::from("write source file failed"),
        }
    }

    pub(crate) fn create_head(&self, table_name: &str, writer: &mut std::io::BufWriter<std::fs::File>) {
        writeln!(writer, r##"#include "{}.h"

#include <fmt/format.h>
#include <pqxx/row>

namespace {}
{{
"##, table_name, self.config_model.namespace()).expect(&self.error_message);
    }

    pub(crate) fn create_from_database_row(
        &self,
        column_list: &[DatabaseColumnMeta],
        writer: &mut std::io::BufWriter<std::fs::File>) {
        writeln!(writer, r##"void SensorParamRow::FromDatabaseRow(const pqxx::row& row)
{{
{0}if (bit_.none())
{0}{{
{1}bit_.set();
{0}}}"##, self.indent._1, self.indent._2).expect(&self.error_message);
        for column in column_list {
            let cpp_type_string = self.type_mapping.database_to_cpp_mapping(&column.data_type);
            writeln!(writer, "{0}if (has_{1}()) set_{1}(row[\"{1}\"].as<{2}>());",
                self.indent._1, column.column_name, cpp_type_string
            ).expect(&self.error_message);
        }
        writeln!(writer, "}}\n").expect(&self.error_message);
    }

    pub(crate) fn create_set_valid_columns_1(&self, writer: &mut std::io::BufWriter<std::fs::File>) {
        writeln!(writer, r##"{1}& {1}::SetValidColumns()
{{
{0}bit_.set();
{0}return *this;
}}
"##, self.indent._1, self.row_class_name).expect(&self.error_message);
    }

    pub(crate) fn create_set_valid_columns_2(&self, writer: &mut std::io::BufWriter<std::fs::File>) {
        writeln!(writer, r##"{2}& {2}::SetValidColumns(const std::initializer_list<int>& valid_columns)
{{
{0}for (const int index : valid_columns)
{0}{{
{1}bit_.set(index);
{0}}}
{0}return *this;
}}
"##, self.indent._1, self.indent._2, self.row_class_name).expect(&self.error_message);
    }

    pub(crate) fn create_set_invalid_columns(&self, writer: &mut std::io::BufWriter<std::fs::File>) {
        writeln!(writer, r##"{1}& {1}::SetInvalidColumns()
{{
{0}bit_.reset();
{0}return *this;
}}
"##, self.indent._1, self.row_class_name).expect(&self.error_message);
    }

    pub(crate) fn create_string(
        &self,
        column_list: &[DatabaseColumnMeta],
        writer: &mut std::io::BufWriter<std::fs::File>) {
        writeln!(writer, r##"std::string {0}::String(const int index) const noexcept
{{"##, self.row_class_name).expect(&self.error_message);
        for column in column_list {
            let cpp_type_string = self.type_mapping.database_to_cpp_mapping(&column.data_type);
            let cpp_type = CppType::new(cpp_type_string);
            match cpp_type {
                CppType::String => {
                    writeln!(
                        writer,
                        "{0}if (index == index_{1}) return {1}_;",
                        self.indent._1, column.column_name
                    ).expect(&self.error_message);
                },
                _ => {
                    writeln!(
                        writer,
                        "{0}if (index == index_{1}) return fmt::format(\"{{}}\", {1}_);",
                        self.indent._1, column.column_name
                    ).expect(&self.error_message);
                },
            }
        }
        writeln!(writer, "}}\n").expect(&self.error_message);
    }
}

impl SourceGenerator for PostgresSourceGenerator<'_> {
    fn generate(&mut self, table_name: &str, column_list: &[DatabaseColumnMeta]) {
        println!("generating source for {}", table_name);
        let source_file_full_path = format!("{}/{}.cpp", self.config_model.save_to_path(), table_name);
        println!("source file full path: {}", source_file_full_path);
        self.row_class_name = CommonUtils::generate_row_class_name(String::from(table_name));
        self.table_class_name = CommonUtils::generate_table_class_name(String::from(table_name));
        let file = std::fs::File::create(source_file_full_path).expect("failed to create source file");
        let mut writer = std::io::BufWriter::new(file);
        self.create_head(table_name, &mut writer);
        self.create_from_database_row(column_list, &mut writer);
        self.json_generator.create_from_json(&self.row_class_name, column_list, &mut writer);
        self.create_set_valid_columns_1(&mut writer);
        self.create_set_valid_columns_2(&mut writer);
        self.create_set_invalid_columns(&mut writer);
        self.create_string(column_list, &mut writer);
    }
}
