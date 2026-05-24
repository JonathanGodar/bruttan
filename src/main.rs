mod models;
mod routes;
mod services;

use anyhow::Result;
use dotenvy::dotenv;
use reqwest::{Identity, Url};
use routes::swish_route::swish_callback;
use serde::Deserialize;
use services::Swish;
use std::{env::args, error::Error, fs};
use tokio::join;

use sqlx::postgres::PgPoolOptions;

async fn test_swish() -> anyhow::Result<()> {
    let db_conn_string = dotenvy::var("DATABASE_URL")?;

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_conn_string)
        .await?;

    let pem_path = dotenvy::var("SWISH_PEM_FILE_PATH")?;
    let pk_path = dotenvy::var("SWISH_KEY_FILE_PATH")?;

    let cert = fs::read(pem_path)?;
    let key = fs::read(pk_path)?;

    let id = Identity::from_pkcs8_pem(&cert, &key)?;

    let swish_url = Url::parse("https://mss.cpc.getswish.net/swish-cpcapi/")?;
    let callback_url = Url::parse("https://tun1.ngodag.com/swish_callback")?;

    let a = Swish::new(swish_url, callback_url, id, "1234679304".into())?;

    let res = a
        .create_swish_payment(&db, 10, "Hejsan hoppsan fallerallera")
        .await?;

    a.sync_swish_payment(res.id).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;

    let a = test_swish();

    //
    // let client = ClientBuilder::new().identity(id).build()?;
    //
    // let payload = json!({
    //     "payeeAlias": "1234679304",
    //     "amount": 10,
    //     "callbackUrl": "https://tun1.ngodar.com/api/swish/callback",
    //     "currency": "SEK",
    //     "message": "HELLOWORLD"
    // }
    // );
    //
    // let response = client
    //     .put("https://mss.cpc.getswish.net/swish-cpcapi/api/v2/paymentrequests/0BE6E5B43EB441B6B54FBD2907C4ACEA").json(&payload)
    // //     // .put("https://mss.cpc.getswish.net/swish-cpcapi/api/v2/paymentrequests/OOOGABOOOGAA1234565")
    //     .send()
    //     .await?;
    // // .text()
    // // .await?;
    // dbg!(response.headers());
    // println!("{:?}", response.text().await?);

    let app = Router::new()
        // `GET /` goes to `root`
        .route("/swish_callback/", post(swish_callback));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    let b = axum::serve(listener, app);

    let (a, b) = join!(a, b);

    a?;
    b?;

    // join!(a, b);

    Ok(())
}
