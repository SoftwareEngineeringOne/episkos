use std::future::Future;

use crate::metadata::Category;

use super::Db;

impl Db for Category {
    type Error = super::Error;

    fn write_to_db(&self) -> impl Future<Output = Result<(), Self::Error>> + Send {
        async move {
            println!("Hello, there from the FUTURE");
            Ok(())
        }
    }

    fn from_db() -> impl Future<Output = Result<Self, Self::Error>> + Send {
        async move { todo!() }
    }
}
