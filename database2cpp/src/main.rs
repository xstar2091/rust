use database2cpp::config::Config;
use database2cpp::config::ConfigError;
use database2cpp::postgres;
use database2cpp::generator;

#[tokio::main]
async fn main() {
    let cfg_result = Config::load();
    let cfg = match cfg_result {
        Ok(v) => v,
        Err(e) => match e {
            ConfigError::Io(e) => {panic!("load json file failed: {}", e);}
            ConfigError::Json(e) => {panic!("parse json failed: {}", e);}
        },
    };
    let dsn = format!("host={} port={} user={} password={} dbname={}",
        cfg.database().host(), cfg.database().port(),
        cfg.database().username(), cfg.database().password(),
        cfg.database().database()
    );
    let repo_result = postgres::TableMetaRepo::new(&dsn).await;
    let repo = match repo_result {
        Ok(v) => v,
        Err(e) => { panic!("connect to database failed: {}", e); }
    };
    for table_name in cfg.database().table_list() {
        let cols_result = repo.get_columns(cfg.database().schema(), &table_name).await;
        let cols = match cols_result {
            Ok(v) => v,
            Err(e) => { panic!("query columns from database failed: {}", e); }
        };
        for col in cols {
            println!("{:?}", col);
            generator::CodeGenerator::test();
        }
    }
}
