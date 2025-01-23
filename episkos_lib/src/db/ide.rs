use std::future::Future;

use sqlx::{query, query_as, sqlite::SqliteRow, Row};

use crate::metadata::ide::Ide;

use super::Db;

impl<'c> Db<'c, sqlx::Sqlite, i32> for Ide {
    type Error = super::Error;

    fn write_to_db<E>(&self, executor: E) -> impl Future<Output = Result<(), Self::Error>> + Send
    where
        E: sqlx::Executor<'c, Database = sqlx::Sqlite>,
    {
        async move {
            query("INSERT INTO ide(name, version) VALUES(?, ?)")
                .bind(&self.name)
                .bind(&self.version)
                .execute(executor)
                .await?;
            Ok(())
        }
    }

    fn from_db<E>(id: i32, executor: E) -> impl Future<Output = Result<Self, Self::Error>> + Send
    where
        E: sqlx::Executor<'c, Database = sqlx::Sqlite>,
    {
        async move {
            let row = query("SELECT name, version FROM ide WHERE id = ?")
                .bind(id)
                .fetch_one(executor)
                .await?;

            let name: &str = row.try_get("name")?;
            let version: &str = row.try_get("version")?;

            Ok(Self {
                name: name.to_string(),
                version: version.to_string(),
            })
        }
    }
}
