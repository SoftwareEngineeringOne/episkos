use sqlx::{query, Row};

use crate::metadata::Category;

use super::{BoxedFuture, DatabaseObject, Result};

impl DatabaseObject for Category {
    type Database = sqlx::Sqlite;
    type Id = u32;
    const TABLE_NAME: &str = "category";

    fn write_to_db<'e, 'q, E>(&'e self, executor: E) -> BoxedFuture<'q, Result<()>>
    where
        E: 'e + sqlx::Executor<'e, Database = Self::Database>,
        'e: 'q,
    {
        Box::pin(async move {
            query("INSERT INTO category(name) VALUES(?)")
                .bind(&self.name)
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
            let row = query("SELECT id,name FROM category WHERE id = ?")
                .bind(id)
                .fetch_one(executor)
                .await?;

            let id: i32 = row.try_get("id")?;
            let name: &str = row.try_get("name")?;

            Ok(Category::new(name).with_id(id))
        })
    }
}
