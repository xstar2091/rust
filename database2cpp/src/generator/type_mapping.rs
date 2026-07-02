use std::collections::HashMap;

pub(crate) struct TypeMapping {
    cpp_mapping: HashMap<String, String>,
}

impl TypeMapping {
    pub(crate) fn cpp_mapping(&self, from_type: &str) -> &str {
        let value = self.cpp_mapping.get(from_type);
        match value {
            None => { panic!("unknown type mapping from {}", from_type); },
            Some(v) => { v },
        }
    }

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
            ("interval".to_string(), "std::string".to_string()),
            ("uuid".to_string(), "std::string".to_string()),
            ("json".to_string(), "std::string".to_string()),
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
            cpp_mapping: cpp_type_mapping,
        }
    }
}
