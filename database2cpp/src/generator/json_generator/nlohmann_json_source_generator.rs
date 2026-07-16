use crate::config::{Config, FormaterConfig, ModelConfig};
use crate::generator::generator_trait::JsonSourceGenerator;

pub(crate) struct NlohmannJsonSourceGenerator<'a> {
    formater: &'a FormaterConfig,
    model: &'a ModelConfig,
}

impl<'a> NlohmannJsonSourceGenerator<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            formater: &config.formater(),
            model: &config.model(),
        }
    }
}

impl<'a> JsonSourceGenerator for NlohmannJsonSourceGenerator<'a> {}
