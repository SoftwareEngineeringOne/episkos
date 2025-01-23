use std::future::Future;

use sqlx::Executor;
use thiserror::Error;
struct DatabaseHandler;

pub mod build_system;
pub mod category;
pub mod ide;
pub mod language;

pub trait Db<'c, DB: sqlx::Database, ID>: Sized {
    type Error;

    fn write_to_db<E>(&self, executor: E) -> impl Future<Output = Result<(), Self::Error>> + Send
    where
        E: Executor<'c, Database = DB>;

    fn from_db<E>(id: ID, executor: E) -> impl Future<Output = Result<Self, Self::Error>> + Send
    where
        E: Executor<'c, Database = DB>;
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("db")]
    Db(#[from] sqlx::Error),
    #[error("to be changed")]
    Other,
}
