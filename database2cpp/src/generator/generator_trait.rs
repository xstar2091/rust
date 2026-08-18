use async_trait::async_trait;
use crate::config::DatabaseConfig;

#[derive(Debug)]
pub struct DatabaseColumnMeta {
    pub column_name: String,
    pub data_type: String,
    pub max_length: Option<i32>,
    pub nullable: bool,
    pub default_value: Option<String>,
}

pub(crate) enum CppType {
    Bool,
    Int8,
    UInt8,
    Int16,
    UInt16,
    Int32,
    UInt32,
    Int64,
    UInt64,
    Float,
    Double,
    String,
    Char,
    UnsignedChar,
    Short,
    UnsignedShort,
    Int,
    UnsignedInt,
    Long,
    UnsignedLong,
    LongLong,
    UnsignedLongLong,
}

#[async_trait]
pub(crate) trait DatabaseReader {
    async fn read(&mut self, conf: &DatabaseConfig, table_name: &str);
    fn column_meta_list(&self) -> &Vec<DatabaseColumnMeta>;
}

pub(crate) trait HeaderGenerator {
    fn generate(&mut self, table_name: &str, column_list: &[DatabaseColumnMeta]);
}

pub(crate) trait SourceGenerator {
    fn generate(&mut self, table_name: &str, column_list: &[DatabaseColumnMeta]);
}

pub(crate) trait JsonHeaderGenerator {
    fn create_include(&self, writer: &mut std::io::BufWriter<std::fs::File>);
    fn create_from_json(&self, indent: &str, writer: &mut std::io::BufWriter<std::fs::File>);
    fn create_to_json(&self, indent: &str, writer: &mut std::io::BufWriter<std::fs::File>);
}

pub(crate) trait JsonSourceGenerator {
    fn create_from_json(
        &self,
        class_name: &str,
        column_list: &[DatabaseColumnMeta],
        writer: &mut std::io::BufWriter<std::fs::File>
    );
    fn create_to_json(
        &self,
        class_name: &str,
        column_list: &[DatabaseColumnMeta],
        writer: &mut std::io::BufWriter<std::fs::File>
    );
}

pub(crate) trait DatabaseCppTypeMapping {
    fn database_to_cpp_mapping(&self, database_type: &str) -> &str;
    fn database_to_cpp_type(&self, database_type: &str) -> CppType;
}
