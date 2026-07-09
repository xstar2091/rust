use async_trait::async_trait;
use database2cpp::config::Config;
use database2cpp::config::ConfigError;
use database2cpp::postgres;
use database2cpp::generator;
use database2cpp::generator::CodeGenerator;


#[tokio::main]
async fn main() {
    let cfg_result = Config::load();
    let cfg = match cfg_result {
        Ok(v) => v,
        Err(e) => match e {
            ConfigError::Io(e) => {panic!("load config file failed: {}", e);}
            ConfigError::Json(e) => {panic!("parse config file failed: {}", e);}
        },
    };
    let mut generator = CodeGenerator::new(&cfg).await;
    generator.generate(&cfg).await;
}
