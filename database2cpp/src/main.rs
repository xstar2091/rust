use crate::config::ConfigError;

mod config;

fn main() {
    let cfg_result = config::Config::load("./conf.json");
    let cfg = match cfg_result {
        Ok(v) => v,
        Err(e) => match e {
            ConfigError::Io(e1) => {panic!("load json file failed: {}", e1);}
            ConfigError::Json(e2) => {panic!("parse json failed: {}", e2);}
        },
    };
    println!("{:?}", cfg);
}
