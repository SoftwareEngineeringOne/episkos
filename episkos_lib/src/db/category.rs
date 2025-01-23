use std::future::Future;

use sqlx::{query, Row};

use crate::metadata::Category;

use super::Db;

impl<'c> Db<'c, sqlx::Sqlite, i32> for Category {
    type Error = super::Error;

    fn write_to_db<E>(&self, executor: E) -> impl Future<Output = Result<(), Self::Error>> + Send
    where
        E: sqlx::Executor<'c, Database = sqlx::Sqlite>,
    {
        async move {
            query("INSERT INTO category(name) VALUES(?)")
                .bind(&self.0)
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
            let row = query("SELECT name FROM category WHERE id = ?")
                .bind(id)
                .fetch_one(executor)
                .await?;

            let cat: &str = row.try_get("name")?;

            Ok(cat.into())
        }
    }
}
