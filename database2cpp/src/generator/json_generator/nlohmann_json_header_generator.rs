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

impl<'a> JsonHeaderGenerator for NlohmannJsonHeaderGenerator<'a> {}
