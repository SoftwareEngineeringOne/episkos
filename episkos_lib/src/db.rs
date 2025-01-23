use std::future::Future;

use thiserror::Error;
struct DatabaseHandler;

pub mod category;

pub trait Db: Sized {
    type Error;

    fn write_to_db(&self) -> impl Future<Output = Result<(), Self::Error>> + Send;

    fn from_db() -> impl Future<Output = Result<Self, Self::Error>> + Send;
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("to be changed")]
    Other,
}
