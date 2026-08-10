use std::io::Write;
use crate::config::{Config, FormaterConfig, ModelConfig};
use crate::generator::cpp_type_enum::CppType;
use crate::generator::factory::Factory;
use crate::generator::generator_trait;
use crate::generator::generator_trait::{DatabaseColumnMeta, DatabaseCppTypeMapping, JsonSourceGenerator};
use crate::generator::indent::Indent;

pub(crate) struct NlohmannJsonSourceGenerator<'a> {
    formater: &'a FormaterConfig,
    model: &'a ModelConfig,
    indent: Indent,
    type_mapping: Box<dyn DatabaseCppTypeMapping>,
    error_message: String,
}

impl<'a> NlohmannJsonSourceGenerator<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            formater: &config.formater(),
            model: &config.model(),
            indent: Indent::new(),
            type_mapping: Factory::create_database_to_cpp_type_mapping(config.database().typename()),
            error_message: String::from("write source file failed"),
        }
    }
}

impl<'a> JsonSourceGenerator for NlohmannJsonSourceGenerator<'a> {
    fn create_from_json(
        &self,
        class_name: &str,
        column_list: &[DatabaseColumnMeta],
        writer: &mut std::io::BufWriter<std::fs::File>
    ) {
        write!(writer, "void {}::FromJson(const nlohmann::json& root)\n{{", class_name).expect(&self.error_message);
        for column in column_list {
            let cpp_type_string = self.type_mapping.database_to_cpp_mapping(&column.data_type);
            let cpp_type = CppType::new(&cpp_type_string);
            match cpp_type {
                CppType::Bool => {
                    write!(writer, r##"
{0}if (root.contains("{2}") && root["{2}"].is_boolean())
{1}set_{2}(root["{2}"].get<{3}>());"##,
                           self.indent._1, self.indent._2,
                           column.column_name, cpp_type_string
                    ).expect(&self.error_message);
                }
                CppType::Integer => {
                    write!(writer, r##"
{0}if (root.contains("{2}") && root["{2}"].is_number_integer())
{1}set_{2}(root["{2}"].get<{3}>());"##,
                           self.indent._1, self.indent._2,
                           column.column_name, cpp_type_string
                    ).expect(&self.error_message);
                }
                CppType::Float => {
                    write!(writer, r##"
{0}if (root.contains("{2}") && root["{2}"].is_number_float())
{1}set_{2}(root["{2}"].get<{3}>());"##,
                           self.indent._1, self.indent._2,
                           column.column_name, cpp_type_string
                    ).expect(&self.error_message);
                }
                CppType::String => {
                    write!(writer, r##"
{0}if (root.contains("{2}") && root["{2}"].is_string())
{1}set_{2}(root["{2}"].get<{3}>());"##,
                           self.indent._1, self.indent._2,
                           column.column_name, cpp_type_string
                    ).expect(&self.error_message);
                }
            }
        }
        write!(writer, r##"

{0}if (!root.contains("param") || !root["param"].is_array())
{0}{{
{1}return;
{0}}}
{0}for (const auto& param : root["param"])
{0}{{
{1}if (!param.is_object() ||
{2}!param.contains("name") || !param["name"].is_string() ||
{2}!param.contains("value") || !param["value"].is_string())
{1}{{
{2}continue;
{1}}}
{1}const auto& name = param["name"].get_ref<const std::string&>();
{1}const auto& value = param["value"].get_ref<const std::string&>();
"##, self.indent._1, self.indent._2, self.indent._3).expect(&self.error_message);

        for (i, column) in column_list.iter().enumerate() {
            if i == 0 {
                write!(writer, "{}if ", self.indent._2).expect(&self.error_message);
            } else {
                write!(writer, "{}else if ", self.indent._2).expect(&self.error_message);
            }
            write!(writer, "(name == {0}) set_{0}", column.column_name).expect(&self.error_message);
            let cpp_type = self.type_mapping.database_to_cpp_type(&column.data_type);
            match cpp_type {
                generator_trait::CppType::Bool => {
                    writeln!(writer, r##"(value == "true" || value == "1" || value == "on");"##).expect(&self.error_message);
                }
                generator_trait::CppType::Int8 => {
                    writeln!(writer, "(static_cast<int8_t>(strtol(value.c_str(), nullptr, 10)));").expect(&self.error_message);
                }
                generator_trait::CppType::UInt8 => {
                    writeln!(writer, "(static_cast<uint8_t>(strtol(value.c_str(), nullptr, 10)));").expect(&self.error_message);
                }
                generator_trait::CppType::Int16 => {
                    writeln!(writer, "(static_cast<int16_t>(strtol(value.c_str(), nullptr, 10)));").expect(&self.error_message);
                }
                generator_trait::CppType::UInt16 => {
                    writeln!(writer, "(static_cast<uint16_t>(strtol(value.c_str(), nullptr, 10)));").expect(&self.error_message);
                }
                generator_trait::CppType::Int32 => {
                    writeln!(writer, "(static_cast<int32_t>(strtoll(value.c_str(), nullptr, 10)));").expect(&self.error_message);
                }
                generator_trait::CppType::UInt32 => {
                    writeln!(writer, "(static_cast<uint32_t>(strtoll(value.c_str(), nullptr, 10)));").expect(&self.error_message);
                }
                generator_trait::CppType::Int64 => {
                    writeln!(writer, "(strtoll(value.c_str(), nullptr, 10));").expect(&self.error_message);
                }
                generator_trait::CppType::UInt64 => {
                    writeln!(writer, "(strtoull(value.c_str(), nullptr, 10));").expect(&self.error_message);
                }
                generator_trait::CppType::Float => {
                    writeln!(writer, "(strtof(value.c_str(), nullptr));").expect(&self.error_message);
                }
                generator_trait::CppType::Double => {
                    writeln!(writer, "(strtod(value.c_str(), nullptr));").expect(&self.error_message);
                }
                generator_trait::CppType::String => {
                    writeln!(writer, "(value);").expect(&self.error_message);
                }
                _ => {}
            }
        }
        writeln!(writer, "{0}}}", self.indent._1).expect(&self.error_message);
        writeln!(writer, "}}\n").expect(&self.error_message);
    }
}
