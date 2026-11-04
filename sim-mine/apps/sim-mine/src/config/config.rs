use std::fs;
use serde::Deserialize;
use sim_common::sim::common::ProjectLayoutInfo;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(skip, default = "ProjectLayoutInfo::load")]
    project_layout_info: ProjectLayoutInfo,
    database: Database,
    server: Server,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    name: String,
    user: String,
    password: String,
    host: String,
    port: i32,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    port: i32,
}

impl Config {
    pub fn load() -> Self {
        let layout = ProjectLayoutInfo::load();
        println!("project layout: {:?}", layout);
        let content = match fs::read_to_string(layout.config_file_full_path()) {
            Ok(content) => content,
            Err(err) => panic!("load config file content failed: {}", err),
        };
        let conf = match toml::from_str::<Config>(&content) {
            Ok(conf) => conf,
            Err(err) => panic!("parse config file failed: {}", err),
        };
        conf
    }

    pub fn project_layout_info(&self) -> &ProjectLayoutInfo {
        &self.project_layout_info
    }

    pub fn database(&self) -> &Database {
        &self.database
    }

    pub fn server(&self) -> &Server {
        &self.server
    }
}

impl Database {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn user(&self) -> &str {
        &self.user
    }

    pub fn password(&self) -> &str {
        &self.password
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> i32 {
        self.port
    }
}

impl Server {
    pub fn port(&self) -> i32 {
        self.port
    }
}
