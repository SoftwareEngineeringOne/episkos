use sqlx::{query, Row};

use crate::metadata::BuildSystem;

use super::{BoxedFuture, DatabaseObject, Result};

impl DatabaseObject for BuildSystem {
    type Database = sqlx::Sqlite;

    type Id = u32;

    fn write_to_db<'e, E>(&self, executor: E) -> BoxedFuture<'e, Result<()>>
    where
        E: 'e + sqlx::Executor<'e, Database = Self::Database>,
    {
        Box::pin(async move {
            query("INSERT INTO build_system(name, version) VALUES(?, ?)")
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
            let row = query("SELECT name, version FROM build_system WHERE id = ?")
                .bind(id)
                .fetch_one(executor)
                .await?;

            let name: &str = row.try_get("name")?;
            let version: &str = row.try_get("version")?;

            Ok(cat.into())
        })
    }
}
