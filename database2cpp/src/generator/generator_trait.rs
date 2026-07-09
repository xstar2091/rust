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

#[async_trait]
pub(crate) trait DatabaseReader {
    async fn read(&mut self, conf: &DatabaseConfig, table_name: &str);
    fn column_meta_list(&self) -> &Vec<DatabaseColumnMeta>;
}

pub(crate) trait HeaderGenerator {
    
}

pub(crate) trait SourceGenerator {
    
}

pub(crate) trait JsonGenerator {

}
