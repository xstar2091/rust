use crate::config::ConfigError;

mod config;

fn main() {
    let cfg_result = config::Config::load();
    let cfg = match cfg_result {
        Ok(v) => v,
        Err(e) => match e {
            ConfigError::Io(e) => {panic!("load json file failed: {}", e);}
            ConfigError::Json(e) => {panic!("parse json failed: {}", e);}
        },
    };
    println!("{:?}", cfg);
}
