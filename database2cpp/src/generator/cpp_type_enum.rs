pub(crate) enum CppType {
    Integer,
    Float,
    String,
}

impl CppType {
    pub(crate) fn new(cpp_type: &str) -> Self {
        if cpp_type == "std::string" {
            return Self::String;
        } else if cpp_type == "double" || cpp_type == "float" {
            return Self::Float;
        }
        Self::Integer
    }
}
