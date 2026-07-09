use async_trait::async_trait;
use tokio_postgres::{Client, NoTls};
use crate::config::DatabaseConfig;
use crate::generator::generator_trait::{DatabaseColumnMeta, DatabaseReader};

pub(crate) struct PostgresReader {
    client: Client,
    pub(crate) column_meta_list: Vec<DatabaseColumnMeta>
}

impl PostgresReader {
    pub(crate) async fn new(conf: &DatabaseConfig) -> Self {
        let dsn = format!("host={} port={} user={} password={} dbname={}",
                          conf.host(), conf.port(),
                          conf.username(), conf.password(),
                          conf.database()
        );
        let connect_result = tokio_postgres::connect(&dsn, NoTls).await;
        let (client, connection) = match connect_result {
            Ok((client, connection)) => (client, connection),
            Err(e) => panic!("connect to database failed: {}", e)
        };
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });
        PostgresReader {
            client,
            column_meta_list: Vec::new()
        }
    }
}

#[async_trait]
impl DatabaseReader for PostgresReader {
    async fn read(&mut self, conf: &DatabaseConfig, table_name: &str) {
        self.column_meta_list.clear();
        let schema: &str = conf.schema();
        let table: &str = table_name;
        let query_rows_result = self.client.query(
            r#"
                SELECT
                    column_name,
                    data_type,
                    character_maximum_length,
                    is_nullable,
                    column_default
                FROM information_schema.columns
                WHERE table_schema = $1
                  AND table_name = $2
                ORDER BY ordinal_position
                "#,
            &[&schema, &table],
        ).await;
        let rows = match query_rows_result {
            Ok(v) => v,
            Err(e) => panic!("query database column info failed: {}", e),
        };
        for row in rows {
            self.column_meta_list.push(DatabaseColumnMeta{
                column_name: row.get(0),
                data_type: row.get(1),
                max_length: row.get(2),
                nullable: row.get::<_, String>(3) == "YES",
                default_value: row.get(4),
            });
        }
    }

    fn column_meta_list(&self) -> &Vec<DatabaseColumnMeta> {
        &self.column_meta_list
    }
}
