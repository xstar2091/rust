use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::OnceLock;

#[derive(Deserialize, Debug)]
pub struct GlobalConfig {
    pub host: String,
    pub url_path: UrlPathConfig,
    pub database_connection_string: String,
    pub scene_list: Vec<SceneListItemConfig>,
    pub tag_list: Vec<TagListItemConfig>,
    pub scene_tag_relation_list: Vec<SceneTagRelationItemConfig>,
}

#[derive(Deserialize, Debug)]
pub struct UrlPathConfig {
    // pub scene_list: String,
    pub scene_tag_create: String,
    pub scene_tag_update: String,
    pub scene_tag_with_id: String,
    pub scene_tag_list: String,
    pub scene_tag_relate: String,
    pub from_scene_to_tag: String,
    pub from_tag_to_scene: String,
}

#[derive(Deserialize, Debug)]
pub struct SceneListItemConfig {
    pub id: i32,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TagListItemConfig {
    pub id: i64,
    pub name: String,
    pub description: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SceneTagRelationItemConfig {
    pub scene_id: i64,
    pub tag_id_list: Vec<i64>,
}

static GLOBAL_CONFIG: OnceLock<GlobalConfig> = OnceLock::new();

pub fn get_global_config() -> &'static GlobalConfig {
    GLOBAL_CONFIG.get_or_init(||{
        GlobalConfig::load()
    })
}

impl GlobalConfig {
    pub fn load() -> Self {
        let json_file_path = "./conf.local.json";
        let content = match fs::read_to_string(json_file_path) {
            Ok(content) => content,
            Err(err) => panic!("load config file:{}, {}", json_file_path, err),
        };
        match serde_json::from_str(&content) {
            Ok(config) => config,
            Err(err) => panic!("load json from config file failed, {}", err),
        }
    }
}
