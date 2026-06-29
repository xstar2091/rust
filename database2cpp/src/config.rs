use serde::Deserialize;
use std::fs;
use std::io::{Error, ErrorKind};

#[derive(Debug)]
pub enum ConfigError {
    Io(Error),
    Json(serde_json::Error),
}

#[derive(Deserialize, Debug)]
pub struct Config {
    database: DatabaseConfig,
    model: ModelConfig,
    formater: FormaterConfig,
}

#[derive(Deserialize, Debug)]
pub struct DatabaseConfig {
    host: String,
    port: u16,
    username: String,
    password: String,
    database: String,
}

#[derive(Deserialize, Debug)]
pub struct ModelConfig {
    save_to_path: String,
    namespace: String,
    #[serde(rename = "table")]
    table_list: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct FormaterConfig {
    indent_char: char,
    indent_width: u8,
    line_width: u16
}

impl From<Error> for ConfigError {
    fn from(e: Error) -> Self {
        ConfigError::Io(e)
    }
}

impl From<serde_json::Error> for ConfigError {
    fn from(e: serde_json::Error) -> Self {
        ConfigError::Json(e)
    }
}

impl Config {
    pub fn database(&self) -> &DatabaseConfig {
        &self.database
    }

    pub fn model(&self) -> &ModelConfig {
        &self.model
    }

    pub fn load() -> Result<Self, ConfigError> {
        let args: Vec<String> = std::env::args().collect();
        let json_file_path = match args.as_slice() {
            [_] => "./conf.json",
            [_, path] => path,
            _ => {
                return Err(ConfigError::Io(Error::new(
                    ErrorKind::InvalidInput,
                    "usage: app [config.json]"
                )))
            }
        };

        let content = fs::read_to_string(json_file_path)?;
        let cfg: Config = serde_json::from_str(&content)?;
        Ok(cfg)
    }
}

impl DatabaseConfig {
    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn password(&self) -> &str {
        &self.password
    }

    pub fn database(&self) -> &str {
        &self.database
    }
}

impl ModelConfig {
    pub fn save_to_path(&self) -> &str {
        &self.save_to_path
    }

    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    pub fn table_list(&self) -> &[String] {
        &self.table_list
    }
}

impl FormaterConfig {
    pub fn indent_char(&self) -> char {
        self.indent_char
    }

    pub fn indent_width(&self) -> u8 {
        self.indent_width
    }

    pub fn line_width(&self) -> u16 {
        self.line_width
    }
}
