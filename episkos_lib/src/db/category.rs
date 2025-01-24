use sqlx::{query, Row};

use crate::metadata::Category;

use super::{BoxedFuture, DatabaseObject, Result};

impl DatabaseObject for Category {
    type Database = sqlx::Sqlite;

    type Id = u32;

    fn write_to_db<'e, E>(&self, executor: E) -> BoxedFuture<'e, Result<()>>
    where
        E: 'e + sqlx::Executor<'e, Database = Self::Database>,
    {
        Box::pin(async move {
            query("INSERT INTO category(name) VALUES(?)")
                .bind(&self.0)
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
            let row = query("SELECT name FROM category WHERE id = ?")
                .bind(id)
                .fetch_one(executor)
                .await?;

            let cat: &str = row.try_get("name")?;

            Ok(cat.into())
        })
    }
}
