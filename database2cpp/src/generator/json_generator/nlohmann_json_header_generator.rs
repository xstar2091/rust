use std::io::Write;
use crate::config::{Config, FormaterConfig, ModelConfig};
use crate::generator::generator_trait::JsonHeaderGenerator;

pub(crate) struct NlohmannJsonHeaderGenerator<'a> {
    formater: &'a FormaterConfig,
    model: &'a ModelConfig,
}

impl<'a> NlohmannJsonHeaderGenerator<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            formater: &config.formater(),
            model: &config.model(),
        }
    }
}

impl<'a> JsonHeaderGenerator for NlohmannJsonHeaderGenerator<'a> {
    fn create_include(&self, writer: &mut std::io::BufWriter<std::fs::File>) {
        writeln!(writer, "#include <nlohmann/json.hpp>").expect("write header file failed")
    }

    fn create_from_json(&self, indent: &str, writer: &mut std::io::BufWriter<std::fs::File>) {
        writeln!(writer, "{}void FromJson(const nlohmann::json& root);", indent).expect("write header file failed")
    }

    fn create_to_json(&self, indent: &str, writer: &mut std::io::BufWriter<std::fs::File>) {
        writeln!(writer, "{}[[nodiscard]] nlohmann::json ToJson() const;", indent).expect("write header file failed")
    }
}
