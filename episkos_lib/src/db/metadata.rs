use uuid::Uuid;

use crate::metadata::Metadata;

use super::{BoxedFuture, DatabaseObject, Result};

impl DatabaseObject for Metadata {
    type Id = Uuid;
    const TABLE_NAME: &str = "metadata";

    fn write_to_db<'e, 'q, E>(&'e self, executor: E) -> BoxedFuture<'q, Result<()>>
    where
        'e: 'q,
        E: 'e + sqlx::Executor<'e, Database = Self::Database>,
    {
        todo!()
    }

    fn from_db<'e, E>(id: Self::Id, executor: E) -> BoxedFuture<'e, Result<Self>>
    where
        E: 'e + sqlx::Executor<'e, Database = Self::Database>,
    {
        todo!()
    }
}
