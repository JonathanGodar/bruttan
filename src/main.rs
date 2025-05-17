mod services;

use anyhow::Result;
use axum::{Router, routing::get};
use reqwest::{ClientBuilder, Identity};
use serde_json::json;
use services::testytest;
use std::{collections::HashMap, env, error::Error, fs};

use sqlx::{Connection, SqliteConnection, prelude::*};

// #[derive(Debug, FromRow)]
// struct ItemGroup {
//     id: i64,
//     description: String,
//     // #[serde(skip)]
//     // #[serde(skip)]
//     required: bool,
//     single_answer: bool,
//     // #[sqlx(skip)]
//     // item: Vec<Item>,
// }
//
// #[derive(Debug, FromRow)]
// struct Item {
//     id: i64,
//     price: Option<i64>,
//     description: String,
// }
//
// async fn add_items_to_item_group() {}
//
// async fn create_item_group(
//     mut c: SqliteConnection,
//     description: String,
//     required: bool,
//     single_answer: bool,
// ) -> Result<ItemGroup> {
//     todo!();
//     // let a = sqlx::query_as!(
//     //     ItemGroup,
//     //     "INSERT INTO item_group (description, required, single_answer) VALUES (?, ?, ?) RETURNING *",
//     //     description,
//     //     required,
//     //     single_answer
//     // )
//     // // .execute(&mut c)
//     // .await?;
//     //
// }
//
// async fn get_item(mut c: SqliteConnection, id: i64) -> Result<Item> {
//     let item = sqlx::query_as!(
//         Item,
//         "
//         SELECT * FROM item WHERE (id = ?)
//         ",
//         id
//     )
//     .fetch_one(&mut c)
//     .await?;
//
//     Ok(item)
// }
//
// async fn add_item(mut c: SqliteConnection, description: &str) -> Result<Item> {
//     let itm = sqlx::query_as!(
//         Item,
//         "INSERT INTO item (description) VALUES (?1) RETURNING *",
//         description
//     )
//     // .execute(&mut c)
//     .fetch_one(&mut c)
//     .await?;
//
//     Ok(itm)
// }
//
// async fn root() -> &'static str {
//     "Hejsan svejsan cooling"
// }
//

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let a = dotenvy::dotenv()?;

    let pem_path = dotenvy::var("SWISH_PEM_FILE_PATH")?;
    let pk_path = dotenvy::var("SWISH_KEY_FILE_PATH")?;

    dbg!(&pem_path);
    dbg!(&pk_path);

    let cert = fs::read(pem_path)?;
    let key = fs::read(pk_path)?;

    println!("hello world!");

    let id = Identity::from_pkcs8_pem(&cert, &key)?;

    let client = ClientBuilder::new().identity(id).build()?;

    let payload = json!({
        "payeeAlias": "1234679304",
        "amount": 10,
        "callbackUrl": "https://tun1.ngodar.com/api/swish/callback",
        "currency": "SEK",
        "message": "HELLOWORLD"
    }
    );

    let response = client
        .put("https://mss.cpc.getswish.net/swish-cpcapi/api/v2/paymentrequests/0BE6E5B43EB441B6B54FBD2907C4ACEA").json(&payload)
    //     // .put("https://mss.cpc.getswish.net/swish-cpcapi/api/v2/paymentrequests/OOOGABOOOGAA1234565")
        .send()
        .await?;
    // .text()
    // .await?;
    dbg!(response.headers());
    println!("{:?}", response.text().await?);

    // let app = Router::new()
    //     // `GET /` goes to `root`
    //     .route("/", get(root));

    // // run our app with hyper, listening globally on port 3000
    // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // axum::serve(listener, app).await.unwrap();
    //
    Ok(())
    // let conn = SqliteConnection::connect(&(env::var("DATABASE_URL")?)).await?;
    //
    // dbg!(add_item(conn, "Dubbel nubbe").await?);
    //
    // Ok(())
}
