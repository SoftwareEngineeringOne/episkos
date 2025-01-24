use sqlx::{query, Row};

use crate::metadata::Language;

use super::{BoxedFuture, DatabaseObject, Result};

impl DatabaseObject for Language {
    type Database = sqlx::Sqlite;
    type Id = u32;
    const TABLE_NAME: &str = "language";

    fn write_to_db<'e, 'q, E>(&'e self, executor: E) -> BoxedFuture<'q, Result<()>>
    where
        E: 'e + sqlx::Executor<'e, Database = Self::Database>,
        'e: 'q,
    {
        Box::pin(async move {
            query("INSERT INTO language(name, version) VALUES(?, ?)")
                .bind(&self.name)
                .bind(&self.version)
                .execute(executor)
                .await?;
            Ok(())
        })
    }

    fn from_db<'e, E>(id: Self::Id, executor: E) -> BoxedFuture<'e, Result<Self>>
    where
        E: 'e + sqlx::Executor<'e, Database = Self::Database>,
    {
        Box::pin(async move {
            let row = query("SELECT id, name, version FROM language WHERE id = ?")
                .bind(id)
                .fetch_one(executor)
                .await?;

            let id: i32 = row.try_get("id")?;
            let name: &str = row.try_get("name")?;
            let version: &str = row.try_get("version")?;

            Ok(Language::new(name, version).with_id(id))
        })
    }
}
