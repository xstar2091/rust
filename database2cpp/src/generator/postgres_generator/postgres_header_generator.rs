use std::io::Write;
use crate::config::{Config, FormaterConfig, ModelConfig};
use crate::generator::common_utils::CommonUtils;
use crate::generator::cpp_type_enum::CppType;
use crate::generator::factory::Factory;
use crate::generator::generator_trait::{DatabaseColumnMeta, DatabaseCppTypeMapping, HeaderGenerator, JsonHeaderGenerator};
use crate::generator::indent::Indent;

pub(crate) struct PostgresHeaderGenerator<'a> {
    config_formater: &'a FormaterConfig,
    config_model: &'a ModelConfig,
    indent : Indent,
    json_generator: Box<dyn JsonHeaderGenerator + 'a>,
    type_mapping: Box<dyn DatabaseCppTypeMapping>,
    class_name: String,
    error_message: String,
}

impl<'a> PostgresHeaderGenerator<'a> {
    pub(crate) fn new(config: &'a Config) -> Self {
        Self {
            config_formater: config.formater(),
            config_model: config.model(),
            indent: Indent::new(),
            json_generator: Factory::create_json_header_generator(config),
            type_mapping: Factory::create_database_to_cpp_type_mapping(config.database().typename()),
            class_name: String::new(),
            error_message: String::from("write header file failed"),
        }
    }

    fn create_head(&self, writer: &mut std::io::BufWriter<std::fs::File>) {
        writeln!(writer, r##"pragma once

#include <array>
#include <bitset>
#include <string>
#include <vector>"##).expect(&self.error_message);
        self.json_generator.create_include(writer);
        writeln!(writer, "#include \"simcommon/error_trace_info.h\"").expect(&self.error_message);
        writeln!(writer, r##"
namespace pqxx
{{
class row;
}}

namespace {}
{{

class {}
{{
"##, self.config_model.namespace(), self.class_name).expect(&self.error_message);
    }

    fn create_member(&self, column_list: &[DatabaseColumnMeta], writer: &mut std::io::BufWriter<std::fs::File>) {
        for column in column_list {
            let cpp_type_string = self.type_mapping.database_to_cpp_mapping(&column.data_type);
            let cpp_type = CppType::new(cpp_type_string);
            match cpp_type {
                CppType::Integer => {
                    writeln!(writer, "{}{} {}_ = 0;",
                             self.indent._1, cpp_type_string, column.column_name).expect(&self.error_message);
                }
                CppType::Float => {
                    writeln!(writer, "{}{} {}_ = 0.0;",
                             self.indent._1, cpp_type_string, column.column_name).expect(&self.error_message);
                }
                CppType::String => {
                    writeln!(writer, "{}{} {}_;",
                             self.indent._1, cpp_type_string, column.column_name).expect(&self.error_message);
                }
            }
        }
        writeln!(writer, "").expect(&self.error_message);
    }

    fn create_column_index(&self, column_list: &[DatabaseColumnMeta], writer: &mut std::io::BufWriter<std::fs::File>) {
        writeln!(writer, "{}enum\n{}{{", self.indent._1, self.indent._1).expect(&self.error_message);
        for column in column_list {
            writeln!(writer, "{}index_{},", self.indent._2, column.column_name).expect(&self.error_message);
        }
        writeln!(writer, "{}}};", self.indent._1).expect(&self.error_message);
        writeln!(writer, "{}std::bitset<index_total_count> bit_;\n", self.indent._1).expect(&self.error_message);
    }

    fn create_ctor(&self, writer: &mut std::io::BufWriter<std::fs::File>) {
        writeln!(writer, r##"{0}{1}() = default;
{0}{1}(const {1}&) = default;
{0}{1}({1}&&) noexcept = default;
{0}{1}& operator=(const {1}&) = default;
{0}{1}& operator=({1}&&) noexcept = default;
{0}~{1}() = default;
"##, self.indent._1, self.class_name).expect(&self.error_message);
    }

    fn create_column_struct(&self, column_list: &[DatabaseColumnMeta], writer: &mut std::io::BufWriter<std::fs::File>) {
        writeln!(writer, r##"{0}struct Column
{0}{{"##, self.indent._1).expect(&self.error_message);

        for column in column_list {
            writeln!(writer, "{0}static constexpr std::string_view {1} = \"{1}\"",
                self.indent._2, column.column_name
            ).expect(&self.error_message);
        }

        writeln!(writer, "{0}static constexpr std::array<std::string_view, index_total_count> placeholders = {{",
            self.indent._2
        ).expect(&self.error_message);
        for i in 1..=column_list.len() {
            writeln!(writer, "{}\"${}\",", self.indent._3, i).expect(&self.error_message);
        }
        writeln!(writer, "{}}};", self.indent._2).expect(&self.error_message);

        writeln!(writer, "{}static constexpr std::array<std::string_view, index_total_count> columns = {{",
            self.indent._2
        ).expect(&self.error_message);
        for column in column_list {
            writeln!(writer, "{}{},", self.indent._3, column.column_name).expect(&self.error_message);
        }
        writeln!(writer, "{}}};", self.indent._2).expect(&self.error_message);

        writeln!(writer, "{}static constexpr int total_count = index_total_count;", self.indent._2).expect(&self.error_message);
        writeln!(writer, "{}}};", self.indent._1).expect(&self.error_message);
    }
}

impl<'a> HeaderGenerator for PostgresHeaderGenerator<'a> {
    fn generate(&mut self, table_name: &str, column_list: &[DatabaseColumnMeta]) {
        println!("generating header for {}", table_name);
        let header_file_full_path = format!("{}/{}.h", self.config_model.save_to_path(), table_name);
        println!("header file full path: {}", header_file_full_path);
        self.class_name = CommonUtils::generate_class_name(String::from(table_name));
        let file = std::fs::File::create(header_file_full_path).expect("failed to create header file");
        let mut writer = std::io::BufWriter::new(file);
        self.create_head(&mut writer);
        self.create_member(column_list, &mut writer);
        self.create_column_index(column_list, &mut writer);
        writeln!(writer, "public:").expect(&self.error_message);
        self.create_ctor(&mut writer);
        self.create_column_struct(column_list, &mut writer);
        writeln!(writer, "{}static constexpr std::string_view table_name = \"{}\";",
            self.indent._1, table_name
        ).expect(&self.error_message);
        writeln!(writer, "").expect(&self.error_message);
    }
}
