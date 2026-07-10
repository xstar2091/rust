use crate::generator::generator::Generator;
use crate::generator::generator_trait::{DatabaseCppTypeMapping, HeaderGenerator, JsonHeaderGenerator};

pub(crate) struct PostgresHeaderGenerator<'a> {
    generator: &'a Generator
}

impl<'a> PostgresHeaderGenerator<'a> {
    pub(crate) fn new(generator: &'a Generator) -> Self {
        Self {
            generator
        }
    }
}

impl HeaderGenerator for PostgresHeaderGenerator<'_> {}
