use std::{
    collections::HashMap,
    fs,
    sync::{Arc, RwLock},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct User {
    pub id: i64,
    pub name: String,
    pub age: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Order {
    pub id: i64,
    pub date: String,
    pub time: String,
}

#[derive(Clone)]
pub struct AppState {
    pub(crate) user: Arc<RwLock<HashMap<i64, User>>>,
    pub(crate) order: Arc<RwLock<HashMap<i64, Order>>>,
}

impl AppState
{
    pub fn new(data_file_path: &str) -> Self {
        let content = match fs::read_to_string(data_file_path) {
            Ok(content) => content,
            Err(error) => {
                tracing::error!(
                    path = data_file_path,
                    error = %error,
                    "读取数据文件失败，进程退出"
                );
                std::process::exit(1);
            },
        };

        #[derive(Debug, Deserialize)]
        struct TomlData {
            #[serde(default)]
            user: Vec<User>,
            #[serde(default)]
            order: Vec<Order>,
        }
        let data: TomlData = match toml::from_str(&content) {
            Ok(d) => d,
            Err(error) => {
                tracing::error!(
                    path = data_file_path,
                    error = %error,
                    "解析 TOML 失败，进程退出"
                );
                std::process::exit(1);
            },
        };

        let mut user_map = HashMap::with_capacity(data.user.len());
        for u in data.user {
            if user_map.insert(u.id, u.clone()).is_some() {
                tracing::error!(
                    id = u.id,
                    "重复的 user id"
                );
            }
        }

        let mut order_map = HashMap::with_capacity(data.order.len());
        for o in data.order {
            if order_map.insert(o.id, o.clone()).is_some() {
                tracing::error!(
                    id = o.id,
                    "重复的 order id"
                );
            }
        }

        tracing::info!(
            users = user_map.len(),
            orders = order_map.len(),
            path = data_file_path,
            "数据加载完成"
        );

        Self {
            user: Arc::new(RwLock::new(user_map)),
            order: Arc::new(RwLock::new(order_map)),
        }
    }
}
