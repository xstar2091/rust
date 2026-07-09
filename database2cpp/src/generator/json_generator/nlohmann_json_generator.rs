use crate::config::{Config, FormaterConfig, ModelConfig};
use crate::generator::generator_trait::JsonGenerator;

pub(crate) struct NlohmannJsonGenerator<'a> {
    formater: &'a FormaterConfig,
    model: &'a ModelConfig,
}

impl<'a> NlohmannJsonGenerator<'a> {
    pub fn new(config: &'a Config) -> Self {
        NlohmannJsonGenerator {
            formater: &config.formater(),
            model: &config.model(),
        }
    }
}

impl<'a> JsonGenerator for NlohmannJsonGenerator<'a> {}

