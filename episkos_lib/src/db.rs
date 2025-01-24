use sqlx::{query, AnyExecutor};
use thiserror::Error;
struct DatabaseHandler;

pub mod build_system;
pub mod category;
pub mod ide;
pub mod language;
pub mod metadata;

type Result<T> = std::result::Result<T, Error>;

pub trait DatabaseObject<'c>: Send + Sized {
    type Id: sqlx::Type<sqlx::Any> + sqlx::Encode<'c, sqlx::Any>;
    const TABLE_NAME: &'c str;

    async fn write_to_db(&self, executor: impl AnyExecutor) -> Result<()>;

    async fn from_db(id: Self::Id, executor: impl AnyExecutor) -> Result<Self>;

    fn id(&self) -> Result<Self::Id>;

    async fn exists_in_db(&self, executor: impl AnyExecutor<'_>) -> Result<bool> {
        let result = query("SELECT count(id) FROM ? WHERE id = ?")
            .bind(&Self::TABLE_NAME)
            .bind(self.id())
            .execute(executor)
            .await?;

        Ok(false)
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("db")]
    Db(#[from] sqlx::Error),
    #[error("to be changed")]
    Other,
}
