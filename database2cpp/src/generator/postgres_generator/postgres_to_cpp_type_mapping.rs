use std::collections::HashMap;
use crate::generator::generator_trait::{CppType, DatabaseCppTypeMapping};

pub(crate) struct PostgresToCppTypeMapping {
    mapping: HashMap<String, String>,
}

impl PostgresToCppTypeMapping {
    pub(crate) fn new() -> Self {
        let cpp_type_mapping = vec![
            ("smallint".to_string(), "int16_t".to_string()),
            ("integer".to_string(), "int32_t".to_string()),
            ("bigint".to_string(), "int64_t".to_string()),
            ("serial".to_string(), "int32_t".to_string()),
            ("bigserial".to_string(), "int64_t".to_string()),
            ("real".to_string(), "float".to_string()),
            ("double precision".to_string(), "double".to_string()),
            ("numeric".to_string(), "std::string".to_string()),
            ("boolean".to_string(), "bool".to_string()),
            ("char(n)".to_string(), "std::string".to_string()),
            ("varchar(n)".to_string(), "std::string".to_string()),
            ("text".to_string(), "std::string".to_string()),
            ("bytea".to_string(), "std::string".to_string()),
            ("date".to_string(), "std::string".to_string()),
            ("time".to_string(), "std::string".to_string()),
            ("timestamp".to_string(), "std::string".to_string()),
            ("timestamptz".to_string(), "std::string".to_string()),
            ("timestamp without time zone".to_string(), "std::string".to_string()),
            ("interval".to_string(), "std::string".to_string()),
            ("uuid".to_string(), "std::string".to_string()),
            ("json_generator".to_string(), "std::string".to_string()),
            ("jsonb".to_string(), "std::string".to_string()),
            ("inet".to_string(), "std::string".to_string()),
            ("cidr".to_string(), "std::string".to_string()),
            ("macaddr".to_string(), "std::string".to_string()),
            ("enum".to_string(), "int".to_string()),
            ("array".to_string(), "std::string".to_string()),
            ("hstore".to_string(), "std::string".to_string()),
        ]
            .into_iter()
            .collect();
        Self {
            mapping: cpp_type_mapping,
        }
    }
}

impl DatabaseCppTypeMapping for PostgresToCppTypeMapping {
    fn database_to_cpp_mapping(&self, database_type: &str) -> &str {
        let result = self.mapping.get(database_type);
        match result {
            None => { panic!("unknown database type {}", database_type); }
            Some(v) => v,
        }
    }

    fn database_to_cpp_type(&self, database_type: &str) -> CppType {
        let type_str = self.database_to_cpp_mapping(database_type);
        let cpp_type: CppType;
        if type_str == "bool" {
            cpp_type = CppType::Bool;
        } else if type_str == "int16_t" {
            cpp_type = CppType::Int16;
        } else if type_str == "int32_t" {
            cpp_type = CppType::Int32;
        } else if type_str == "int64_t" {
            cpp_type = CppType::Int64;
        } else if type_str == "float" {
            cpp_type = CppType::Float;
        } else if type_str == "double" {
            cpp_type = CppType::Double;
        } else if type_str == "int" {
            cpp_type = CppType::Int;
        } else {
            cpp_type = CppType::String;
        }
        cpp_type
    }
}
