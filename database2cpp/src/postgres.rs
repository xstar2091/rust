use tokio_postgres::{Client, NoTls, Error};

#[derive(Debug)]
pub struct ColumnMeta {
    pub column_name: String,
    pub data_type: String,
    pub max_length: Option<i32>,
    pub nullable: bool,
    pub default_value: Option<String>,
}

pub struct TableMetaRepo {
    client: Client,
}

impl TableMetaRepo {
    pub async fn new(dsn: &str) -> Result<Self, Error> {
        let (client, connection) = tokio_postgres::connect(dsn, NoTls).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        Ok(Self { client })
    }

    pub async fn get_columns(
        &self,
        schema: &str,
        table: &str,
    ) -> Result<Vec<ColumnMeta>, Error> {
        let rows = self
            .client
            .query(
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
            )
            .await?;

        let mut result = Vec::new();

        for row in rows {
            result.push(ColumnMeta {
                column_name: row.get(0),
                data_type: row.get(1),
                max_length: row.get(2),
                nullable: row.get::<_, String>(3) == "YES",
                default_value: row.get(4),
            });
        }

        Ok(result)
    }
}
