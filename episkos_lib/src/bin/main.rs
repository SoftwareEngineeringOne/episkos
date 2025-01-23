use episkos_lib::{
    db::Db,
    files::File,
    metadata::{Category, Metadata},
};
use sqlx::sqlite::SqlitePoolOptions;
use std::{error::Error, path::Path};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;
    let connection_string = dotenvy::var("DATABASE_URL")?;
    println!("Initializing db...");

    let category: Category = "Fun".into();
    category.write_to_db().await?;

    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&connection_string)
        .await?;
    // let metadata = Metadata::builder()
    //     .title("Hello, World!")
    //     .directory(Path::new("./"))?
    //     .build()?;
    // let path = Path::new(".");
    // println!("Path: {:#?}", path);
    // println!("Absolut: {:#?}", path.canonicalize()?);
    //
    // println!("Metadata: {:#?}", metadata.directory());
    //
    // metadata.write_file(&metadata.directory())?;
    //
    // let read_metadata = Metadata::from_file(Path::new("."))?;
    // println!("Read: {:#?}", read_metadata);

    Ok(())
}
