use std::io::Write;
use crate::config::{Config, FormaterConfig, ModelConfig};
use crate::generator::common_utils::CommonUtils;
use crate::generator::factory::Factory;
use crate::generator::generator_trait::{DatabaseColumnMeta, DatabaseCppTypeMapping, HeaderGenerator, JsonHeaderGenerator};
use crate::generator::indent::Indent;

pub(crate) struct PostgresHeaderGenerator<'a> {
    config_formater: &'a FormaterConfig,
    config_model: &'a ModelConfig,
    indent : Indent,
    json_generator: Box<dyn JsonHeaderGenerator + 'a>,
    type_mapping: Box<dyn DatabaseCppTypeMapping>,
    class_name: String
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
        }
    }

    fn create_head(&self, writer: &mut std::io::BufWriter<std::fs::File>) {
        let error_message = "write header file failed";
        writeln!(writer, r##"pragma once

#include <array>
#include <bitset>
#include <string>
#include <vector>"##).expect(error_message);
        self.json_generator.create_include(writer);
        writeln!(writer, "#include \"simcommon/error_trace_info.h\"").expect(error_message);
        writeln!(writer, r##"
namespace pqxx
{{
class row;
}}

namespace {}
{{

class {}
{{
"##, self.config_model.namespace(), self.class_name).expect(error_message);
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
    }
}
