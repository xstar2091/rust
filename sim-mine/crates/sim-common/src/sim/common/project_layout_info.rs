use std::env;

#[derive(Debug)]
pub struct ProjectLayoutInfo {
    binary_file_name: String,
    binary_file_full_path: String,
    binary_dir_name: String,
    binary_dir_full_path: String,
    config_file_name: String,
    config_file_full_path: String,
    config_dir_name: String,
    config_dir_full_path: String,
    data_dir_name: String,
    data_dir_full_path: String,
    log_file_name: String,
    log_file_full_path: String,
    log_dir_name: String,
    log_dir_full_path: String,
    project_name: String,
    project_full_path: String,
}

impl ProjectLayoutInfo {
    pub fn load() -> Self {
        let mut info = Self {
            binary_file_name: "".to_string(),
            binary_file_full_path: "".to_string(),
            binary_dir_name: "".to_string(),
            binary_dir_full_path: "".to_string(),
            config_file_name: "".to_string(),
            config_file_full_path: "".to_string(),
            config_dir_name: "".to_string(),
            config_dir_full_path: "".to_string(),
            data_dir_name: "".to_string(),
            data_dir_full_path: "".to_string(),
            log_file_name: "".to_string(),
            log_file_full_path: "".to_string(),
            log_dir_name: "".to_string(),
            log_dir_full_path: "".to_string(),
            project_name: "".to_string(),
            project_full_path: "".to_string(),
        };
        let exe_full_path = match env::current_exe() {
            Ok(path_buf) => {
                match path_buf.to_str() {
                    Some(exe_path_str) => info.binary_file_full_path = exe_path_str.to_owned(),
                    None => { panic!("could not get current executable path string: {:?}", path_buf); },
                };
                path_buf
            },
            Err(err) => { panic!("could not get current executable path: {:?}", err); }
        };
        info.binary_file_name = match exe_full_path.file_name().and_then(|name| name.to_str()) {
            Some(name) => name.to_owned(),
            None => panic!("could not get executable file name: {:?}", exe_full_path),
        };
        let project_deploy_path = match exe_full_path.parent() {
            Some(parent_path) => {
                info.binary_dir_full_path = match parent_path.to_str() {
                    Some(v) => v.to_owned(),
                    None => { panic!("could not get bin path string: {:?}", parent_path); },
                };
                info.binary_dir_name = match parent_path.file_name() {
                    Some(v) => match v.to_str() {
                        Some(v) => v.to_string(),
                        None => { panic!("could not get bin dir name"); },
                    },
                    None => panic!("could not get bin dir name"),
                };
                parent_path
            }
            None => { panic!("could not get bin path: {:?}", exe_full_path); }
        };
        match project_deploy_path.parent() {
            Some(deploy_path) => {
                match deploy_path.to_str() {
                    Some(deploy_path_str) => {
                        info.project_full_path = deploy_path_str.to_string();
                    },
                    None => { panic!("could not get project path string"); },
                };
                match deploy_path.file_name() {
                    Some(project_name) => {
                        match project_name.to_str() {
                            Some(project_name_str) => info.project_name = project_name_str.to_string(),
                            None => panic!("could not get project name string"),
                        };
                    },
                    None => panic!("could not get project name"),
                };
            },
            None => panic!("could not get project path: {:?}", project_deploy_path),
        };
        let binary_name_without_ext = match info.binary_file_name.strip_suffix(".exe") {
            Some(v) => v,
            None => &info.binary_file_name,
        };
        info.config_dir_name = "conf".to_string();
        info.config_dir_full_path = format!("{}/{}", info.project_full_path, info.config_dir_name);
        info.config_file_name = format!("{}.conf.toml", binary_name_without_ext);
        info.config_file_full_path = format!("{}/{}", info.config_dir_full_path, info.config_file_name);
        info.data_dir_name = "data".to_string();
        info.data_dir_full_path = format!("{}/{}", info.project_full_path, info.data_dir_name);
        info.log_dir_name = "log".to_string();
        info.log_dir_full_path = format!("{}/{}", info.project_full_path, info.log_dir_name);
        info.log_file_name = format!("{}.log", binary_name_without_ext);
        info.log_file_full_path = format!("{}/{}", info.log_dir_full_path, info.log_file_name);
        info
    }

    pub fn binary_file_name(&self) -> &str {
        &self.binary_file_name
    }

    pub fn binary_file_full_path(&self) -> &str {
        &self.binary_file_full_path
    }

    pub fn binary_dir_name(&self) -> &str {
        &self.binary_dir_name
    }

    pub fn binary_dir_full_path(&self) -> &str {
        &self.binary_dir_full_path
    }

    pub fn config_file_name(&self) -> &str {
        &self.config_file_name
    }

    pub fn config_file_full_path(&self) -> &str {
        &self.config_file_full_path
    }

    pub fn config_dir_name(&self) -> &str {
        &self.config_dir_name
    }

    pub fn config_dir_full_path(&self) -> &str {
        &self.config_dir_full_path
    }

    pub fn data_dir_name(&self) -> &str {
        &self.data_dir_name
    }

    pub fn data_dir_full_path(&self) -> &str {
        &self.data_dir_full_path
    }

    pub fn log_file_name(&self) -> &str {
        &self.log_file_name
    }

    pub fn log_file_full_path(&self) -> &str {
        &self.log_file_full_path
    }

    pub fn log_dir_name(&self) -> &str {
        &self.log_dir_name
    }

    pub fn log_dir_full_path(&self) -> &str {
        &self.log_dir_full_path
    }

    pub fn project_name(&self) -> &str {
        &self.project_name
    }

    pub fn project_full_path(&self) -> &str {
        &self.project_full_path
    }
}
