use std::io::Write;
use std::fs::File;
use std::io::BufWriter;
use serde::Serialize;
use crate::config::{Config, FormaterConfig, ModelConfig};
use crate::generator::factory::Factory;
use crate::generator::generator_trait::{CppType, DatabaseColumnMeta, DatabaseCppTypeMapping, JsonSourceGenerator};
use crate::generator::indent::Indent;

pub(crate) struct JsoncppJsonSourceGenerator<'a> {
    formater: &'a FormaterConfig,
    model: &'a ModelConfig,
    indent: Indent,
    type_mapping: Box<dyn DatabaseCppTypeMapping>,
    error_message: String,
}

impl<'a> JsoncppJsonSourceGenerator<'a> {
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

impl<'a> JsonSourceGenerator for JsoncppJsonSourceGenerator<'a> {
    fn create_from_json(&self, class_name: &str, column_list: &[DatabaseColumnMeta], writer: &mut BufWriter<File>) {
        write!(
            writer,
            "void {0}::FromJson(const Json::Value& root)\n{{",
            class_name
        ).expect(&self.error_message);

        for column in column_list {
            let cpp_type = self.type_mapping.database_to_cpp_type(&column.data_type);
            match cpp_type {
                CppType::Bool => {
                    write!(
                        writer,
                        r##"
{0}if (root.isMember("{1}") && root["{1}"].isBool())
{2}set_{1}(root["{1}"].asBool());"##,
                        self.indent._1, column.column_name, self.indent._2
                    ).expect(&self.error_message);
                }
                CppType::Int8 => {
                    write!(
                        writer,
                        r##"
{0}if (root.isMember("{1}") && root["{1}"].isInt())
{2}set_{1}(static_cast<int8_t>(root["{1}"].asInt()));"##,
                        self.indent._1, column.column_name, self.indent._2
                    ).expect(&self.error_message);
                }
                CppType::UInt8 => {
                    write!(
                        writer,
                        r##"
{0}if (root.isMember("{1}") && root["{1}"].isInt())
{2}set_{1}(static_cast<uint8_t>(root["{1}"].asInt()));"##,
                        self.indent._1, column.column_name, self.indent._2
                    ).expect(&self.error_message);
                }
                CppType::Int16 => {
                    write!(
                        writer,
                        r##"
{0}if (root.isMember("{1}") && root["{1}"].isInt())
{2}set_{1}(static_cast<int16_t>(root["{1}"].asInt()));"##,
                        self.indent._1, column.column_name, self.indent._2
                    ).expect(&self.error_message);
                }
                CppType::UInt16 => {
                    write!(
                        writer,
                        r##"
{0}if (root.isMember("{1}") && root["{1}"].isInt())
{2}set_{1}(static_cast<uint16_t>(root["{1}"].asInt()));"##,
                        self.indent._1, column.column_name, self.indent._2
                    ).expect(&self.error_message);
                }
                CppType::Int32 => {
                    write!(
                        writer,
                        r##"
{0}if (root.isMember("{1}") && root["{1}"].isInt())
{2}set_{1}(root["{1}"].asInt());"##,
                        self.indent._1, column.column_name, self.indent._2
                    ).expect(&self.error_message);
                }
                CppType::UInt32 => {
                    write!(
                        writer,
                        r##"
{0}if (root.isMember("{1}") && root["{1}"].isUInt())
{2}set_{1}(root["{1}"].asUInt());"##,
                        self.indent._1, column.column_name, self.indent._2
                    ).expect(&self.error_message);
                }
                CppType::Int64 => {
                    write!(
                        writer,
                        r##"
{0}if (root.isMember("{1}") && root["{1}"].isInt64())
{2}set_{1}(root["{1}"].asInt64());"##,
                        self.indent._1, column.column_name, self.indent._2
                    ).expect(&self.error_message);
                }
                CppType::UInt64 => {
                    write!(
                        writer,
                        r##"
{0}if (root.isMember("{1}") && root["{1}"].isUInt64())
{2}set_{1}(root["{1}"].asUInt64());"##,
                        self.indent._1, column.column_name, self.indent._2
                    ).expect(&self.error_message);
                }
                CppType::Float => {
                    write!(
                        writer,
                        r##"
{0}if (root.isMember("{1}") && root["{1}"].isDouble())
{2}set_{1}(static_cast<float>(root["{1}"].asDouble()));"##,
                        self.indent._1, column.column_name, self.indent._2
                    ).expect(&self.error_message);
                }
                CppType::Double => {
                    write!(
                        writer,
                        r##"
{0}if (root.isMember("{1}") && root["{1}"].isDouble())
{2}set_{1}(root["{1}"].asDouble());"##,
                        self.indent._1, column.column_name, self.indent._2
                    ).expect(&self.error_message);
                }
                CppType::String => {
                    write!(
                        writer,
                        r##"
{0}if (root.isMember("{1}") && root["{1}"].isString())
{2}set_{1}(root["{1}"].asString());"##,
                        self.indent._1, column.column_name, self.indent._2
                    ).expect(&self.error_message);
                }
                CppType::Char => {
                    write!(
                        writer,
                        r##"
{0}if (root.isMember("{1}") && root["{1}"].isInt())
{2}set_{1}(static_cast<char>(root["{1}"].asInt()));"##,
                        self.indent._1, column.column_name, self.indent._2
                    ).expect(&self.error_message);
                }
                CppType::UnsignedChar => {
                    write!(
                        writer,
                        r##"
{0}if (root.isMember("{1}") && root["{1}"].isInt())
{2}set_{1}(static_cast<unsigned char>(root["{1}"].asInt()));"##,
                        self.indent._1, column.column_name, self.indent._2
                    ).expect(&self.error_message);
                }
                CppType::Short => {
                    write!(
                        writer,
                        r##"
{0}if (root.isMember("{1}") && root["{1}"].isInt())
{2}set_{1}(static_cast<short>(root["{1}"].asInt()));"##,
                        self.indent._1, column.column_name, self.indent._2
                    ).expect(&self.error_message);
                }
                CppType::UnsignedShort => {
                    write!(
                        writer,
                        r##"
{0}if (root.isMember("{1}") && root["{1}"].isInt())
{2}set_{1}(static_cast<unsigned short>(root["{1}"].asInt()));"##,
                        self.indent._1, column.column_name, self.indent._2
                    ).expect(&self.error_message);
                }
                CppType::Int => {
                    write!(
                        writer,
                        r##"
{0}if (root.isMember("{1}") && root["{1}"].isInt())
{2}set_{1}(root["{1}"].asInt());"##,
                        self.indent._1, column.column_name, self.indent._2
                    ).expect(&self.error_message);
                }
                CppType::UnsignedInt => {
                    write!(
                        writer,
                        r##"
{0}if (root.isMember("{1}") && root["{1}"].isUInt())
{2}set_{1}(root["{1}"].asUInt());"##,
                        self.indent._1, column.column_name, self.indent._2
                    ).expect(&self.error_message);
                }
                CppType::Long => {
                    write!(
                        writer,
                        r##"
{0}if (root.isMember("{1}") && root["{1}"].isInt64())
{2}set_{1}(root["{1}"].asInt64());"##,
                        self.indent._1, column.column_name, self.indent._2
                    ).expect(&self.error_message);
                }
                CppType::UnsignedLong => {
                    write!(
                        writer,
                        r##"
{0}if (root.isMember("{1}") && root["{1}"].isUInt64())
{2}set_{1}(root["{1}"].asUInt64());"##,
                        self.indent._1, column.column_name, self.indent._2
                    ).expect(&self.error_message);
                }
                CppType::LongLong => {
                    write!(
                        writer,
                        r##"
{0}if (root.isMember("{1}") && root["{1}"].isInt64())
{2}set_{1}(root["{1}"].asInt64());"##,
                        self.indent._1, column.column_name, self.indent._2
                    ).expect(&self.error_message);
                }
                CppType::UnsignedLongLong => {
                    write!(
                        writer,
                        r##"
{0}if (root.isMember("{1}") && root["{1}"].isUInt64())
{2}set_{1}(root["{1}"].asUInt64());"##,
                        self.indent._1, column.column_name, self.indent._2
                    ).expect(&self.error_message);
                }
            }
        }

        writeln!(
            writer,
            r##"

{0}if (!root.isMember("param") || !root["param"].isArray())
{0}{{
{1}return;
{0}}}
{0}for (const auto& param : root["param"])
{0}{{
{1}if (!param.isObject() ||
{2}!param.isMember("name") || !param["name"].isString() ||
{2}!param.isMember("value") || !param["value"].isString())
{1}{{
{2}continue;
{1}}}
{1}const auto& name = param["name"].asStringRef();
{1}const auto& value = param["value"].asStringRef();
"##, self.indent._1, self.indent._2, self.indent._3
        ).expect(&self.error_message);
        let mut if_else = "";
        column_list.iter().enumerate().for_each(|(_i, column)| {
            if column.column_name == "id" {
                return;
            }
            let cpp_type = self.type_mapping.database_to_cpp_type(&column.data_type);
            match cpp_type {
                CppType::Bool => {
                    write!(
                        writer,
                        r##"{0}{2}if (name == {3})
{1}set_{3}(value == "true" || value == "1" || value == "on");
"##, self.indent._2, self.indent._3, if_else, column.column_name
                    ).expect(&self.error_message);
                }
                CppType::Int8 => {
                    write!(
                        writer,
                        r##"{0}{2}if (name == {3})
{1}set_{3}(static_cast<int8_t>(strtoll(value.c_str(), nullptr, 10)));
"##, self.indent._2, self.indent._3, if_else, column.column_name
                    ).expect(&self.error_message);
                }
                CppType::UInt8 => {
                    write!(
                        writer,
                        r##"{0}{2}if (name == {3})
{1}set_{3}(static_cast<uint8_t>(strtoll(value.c_str(), nullptr, 10)));
"##, self.indent._2, self.indent._3, if_else, column.column_name
                    ).expect(&self.error_message);
                }
                CppType::Int16 => {
                    write!(
                        writer,
                        r##"{0}{2}if (name == {3})
{1}set_{3}(static_cast<int16_t>(strtoll(value.c_str(), nullptr, 10)));
"##, self.indent._2, self.indent._3, if_else, column.column_name
                    ).expect(&self.error_message);
                }
                CppType::UInt16 => {
                    write!(
                        writer,
                        r##"{0}{2}if (name == {3})
{1}set_{3}(static_cast<uint16_t>(strtoll(value.c_str(), nullptr, 10)));
"##, self.indent._2, self.indent._3, if_else, column.column_name
                    ).expect(&self.error_message);
                }
                CppType::Int32 => {
                    write!(
                        writer,
                        r##"{0}{2}if (name == {3})
{1}set_{3}(static_cast<int32_t>(strtoll(value.c_str(), nullptr, 10)));
"##, self.indent._2, self.indent._3, if_else, column.column_name
                    ).expect(&self.error_message);
                }
                CppType::UInt32 => {
                    write!(
                        writer,
                        r##"{0}{2}if (name == {3})
{1}set_{3}(static_cast<uint32_t>(strtoll(value.c_str(), nullptr, 10)));
"##, self.indent._2, self.indent._3, if_else, column.column_name
                    ).expect(&self.error_message);
                }
                CppType::Int64 => {
                    write!(
                        writer,
                        r##"{0}{2}if (name == {3})
{1}set_{3}(strtoll(value.c_str(), nullptr, 10));
"##, self.indent._2, self.indent._3, if_else, column.column_name
                    ).expect(&self.error_message);
                }
                CppType::UInt64 => {
                    write!(
                        writer,
                        r##"{0}{2}if (name == {3})
{1}set_{3}(strtoul(value.c_str(), nullptr, 10));
"##, self.indent._2, self.indent._3, if_else, column.column_name
                    ).expect(&self.error_message);
                }
                CppType::Float => {
                    write!(
                        writer,
                        r##"{0}{2}if (name == {3})
{1}set_{3}(strtof(value.c_str(), nullptr));
"##, self.indent._2, self.indent._3, if_else, column.column_name
                    ).expect(&self.error_message);
                }
                CppType::Double => {
                    write!(
                        writer,
                        r##"{0}{2}if (name == {3})
{1}set_{3}(strtod(value.c_str(), nullptr));
"##, self.indent._2, self.indent._3, if_else, column.column_name
                    ).expect(&self.error_message);
                }
                CppType::String => {
                    write!(
                        writer,
                        r##"{0}{2}if (name == {3})
{1}set_{3}(value);
"##, self.indent._2, self.indent._3, if_else, column.column_name
                    ).expect(&self.error_message);
                }
                CppType::Char => {
                    write!(
                        writer,
                        r##"{0}{2}if (name == {3})
{1}set_{3}(static_cast<char>(strtoll(value.c_str(), nullptr, 10)));
"##, self.indent._2, self.indent._3, if_else, column.column_name
                    ).expect(&self.error_message);
                }
                CppType::UnsignedChar => {
                    write!(
                        writer,
                        r##"{0}{2}if (name == {3})
{1}set_{3}(static_cast<unsigned char>(strtoll(value.c_str(), nullptr, 10)));
"##, self.indent._2, self.indent._3, if_else, column.column_name
                    ).expect(&self.error_message);
                }
                CppType::Short => {
                    write!(
                        writer,
                        r##"{0}{2}if (name == {3})
{1}set_{3}(static_cast<short>(strtoll(value.c_str(), nullptr, 10)));
"##, self.indent._2, self.indent._3, if_else, column.column_name
                    ).expect(&self.error_message);
                }
                CppType::UnsignedShort => {
                    write!(
                        writer,
                        r##"{0}{2}if (name == {3})
{1}set_{3}(static_cast<unsigned short>(strtoll(value.c_str(), nullptr, 10)));
"##, self.indent._2, self.indent._3, if_else, column.column_name
                    ).expect(&self.error_message);
                }
                CppType::Int => {
                    write!(
                        writer,
                        r##"{0}{2}if (name == {3})
{1}set_{3}(static_cast<int>(strtoll(value.c_str(), nullptr, 10)));
"##, self.indent._2, self.indent._3, if_else, column.column_name
                    ).expect(&self.error_message);
                }
                CppType::UnsignedInt => {
                    write!(
                        writer,
                        r##"{0}{2}if (name == {3})
{1}set_{3}(static_cast<unsigned int>(strtoll(value.c_str(), nullptr, 10)));
"##, self.indent._2, self.indent._3, if_else, column.column_name
                    ).expect(&self.error_message);
                }
                CppType::Long => {
                    write!(
                        writer,
                        r##"{0}{2}if (name == {3})
{1}set_{3}(static_cast<long>(strtoll(value.c_str(), nullptr, 10)));
"##, self.indent._2, self.indent._3, if_else, column.column_name
                    ).expect(&self.error_message);
                }
                CppType::UnsignedLong => {
                    write!(
                        writer,
                        r##"{0}{2}if (name == {3})
{1}set_{3}(strtoul(value.c_str(), nullptr, 10));
"##, self.indent._2, self.indent._3, if_else, column.column_name
                    ).expect(&self.error_message);
                }
                CppType::LongLong => {
                    write!(
                        writer,
                        r##"{0}{2}if (name == {3})
{1}set_{3}(strtoll(value.c_str(), nullptr, 10));
"##, self.indent._2, self.indent._3, if_else, column.column_name
                    ).expect(&self.error_message);
                }
                CppType::UnsignedLongLong => {
                    write!(
                        writer,
                        r##"{0}{2}if (name == {3})
{1}set_{3}(strtoul(value.c_str(), nullptr, 10));
"##, self.indent._2, self.indent._3, if_else, column.column_name
                    ).expect(&self.error_message);
                }
            }
            if if_else == "" {
                if_else = "else ";
            }
        });
        writeln!(writer, "{0}}}", self.indent._1).expect(&self.error_message);

        writeln!(writer, "}}\n").expect(&self.error_message);
    }

    fn create_to_json(&self, class_name: &str, column_list: &[DatabaseColumnMeta], writer: &mut BufWriter<File>) {
        println!("aa");
        return;
        todo!()
    }

    fn create_table_class_to_json(&self, class_name: &str, table_name: &str, writer: &mut BufWriter<File>) {
        println!("aa");
        return;
        todo!()
    }
}
