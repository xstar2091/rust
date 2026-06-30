use crate::generator::header_generator::HeaderGenerator;
use crate::generator::source_generator::SourceGenerator;

pub struct CodeGenerator {
    header_generator: HeaderGenerator,
    source_generator: SourceGenerator,
}

impl CodeGenerator {
    pub fn test() {
        println!("CodeGenerator Initialized");
    }
}
