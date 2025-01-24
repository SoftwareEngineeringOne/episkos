use std::{future::Future, pin::Pin};

use sqlx::Executor;
use thiserror::Error;
struct DatabaseHandler;

pub mod build_system;
pub mod category;
pub mod ide;
pub mod language;

type BoxedFuture<'q, T> = Pin<Box<dyn Future<Output = T> + Send + 'q>>;
type Result<T> = std::result::Result<T, Error>;

pub trait DatabaseObject: Send + Sized {
    type Database: sqlx::Database;
    type Id;

    fn write_to_db<'e, E>(&self, executor: E) -> BoxedFuture<'e, Result<()>>
    where
        E: 'e + Executor<'e, Database = Self::Database>;

    fn from_db<'e, E>(id: Self::Id, executor: E) -> BoxedFuture<'e, Result<Self>>
    where
        E: 'e + Executor<'e, Database = Self::Database>;
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("db")]
    Db(#[from] sqlx::Error),
    #[error("to be changed")]
    Other,
}
