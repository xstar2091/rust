use std::io::Write;
use std::fs::File;
use std::io::BufWriter;
use crate::config::{Config, FormaterConfig, ModelConfig};
use crate::generator::generator_trait::JsonHeaderGenerator;

pub(crate) struct JsoncppJsonHeaderGenerator<'a> {
    formater: &'a FormaterConfig,
    model: &'a ModelConfig,
}

impl<'a> JsoncppJsonHeaderGenerator<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            formater: &config.formater(),
            model: &config.model(),
        }
    }
}

impl<'a> JsonHeaderGenerator for JsoncppJsonHeaderGenerator<'a> {
    fn create_include(&self, writer: &mut BufWriter<File>) {
        writeln!(writer, "#include <jsoncpp/json/json.h>").expect("write header file failed")
    }

    fn create_from_json(&self, indent: &str, writer: &mut BufWriter<File>) {
        writeln!(writer, "{0}void FromJson(const Json::Value& root);", indent).expect("write header file failed")
    }

    fn create_to_json(&self, indent: &str, writer: &mut BufWriter<File>) {
        writeln!(writer, "{0}void ToJson(Json::Value& root) const;", indent).expect("write header file failed")
    }
}
