use anyhow::Result;
use std::{env, error::Error};

use sqlx::{Connection, SqliteConnection, prelude::FromRow};

// struct ItemGroup {
//     id: u32,
//     name: String,
// }
//
#[derive(Debug, FromRow)]
struct Item {
    id: u32,
    description: String,
}

async fn get_item(mut c: SqliteConnection, id: i64) {}

async fn add_item(mut c: SqliteConnection, description: &str) -> Result<i64> {
    let itm = sqlx::query_as!(
        Item,
        "INSERT INTO item (description) VALUES (?1)",
        description
    )
    .execute(&mut c)
    // .fetch_one(&mut c)
    .await?;

    Ok(itm.last_insert_rowid())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let conn = SqliteConnection::connect(&(env::var("DATABASE_URL")?)).await?;

    dbg!(add_item(conn, "Dubbel nubbe").await?);

    Ok(())
}
