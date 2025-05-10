use std::error::Error;

use sqlx::{Connection, SqliteConnection};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let conn = SqliteConnection::connect("sqlite::memory:").await?;

    Ok(())
}
