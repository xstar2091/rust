pub mod config;
pub mod postgres;
pub mod generator {
    pub mod code_generator;
    pub use code_generator::CodeGenerator;

    pub(crate) mod common_utils;
    pub(crate) mod factory;
    pub(crate) mod generator_trait;
    pub(crate) mod indent;
    
    pub(crate) mod json_generator {
        pub(crate) mod nlohmann_json_header_generator;
        pub(crate) mod nlohmann_json_source_generator;
    }
    
    pub(crate) mod postgres_generator {
        pub(crate) mod postgres_header_generator;
        pub(crate) mod postgres_reader;
        pub(crate) mod postgres_source_generator;
        pub(crate) mod postgres_to_cpp_type_mapping;
    }
}
