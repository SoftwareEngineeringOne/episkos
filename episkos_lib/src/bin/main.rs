use episkos_lib::{
    db::Db,
    metadata::{Category, Language},
};
use sqlx::{Connection, SqliteConnection};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;
    let connection_string = dotenvy::var("DATABASE_URL")?;
    println!("Initializing db...");

    let mut connection = SqliteConnection::connect(&connection_string).await?;
    // category.write_to_db(&mut connection).await?;
    let category = Category::from_db(1, &mut connection).await?;
    println!("Retrieved Category: {:#?}", category);

    let language = Language::from_db(1, &mut connection).await?;
    println!("Received Language: {:#?}", language);

    // let language = Language {
    //     name: "rust".to_string(),
    //     version: "1.84".to_string(),
    // };
    //
    // language.write_to_db(&mut connection).await?;
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
