mod chapter_event;

mod models;
mod routes;
mod services;

use std::{error::Error, fs};

use anyhow::Result;
use dotenvy::{dotenv, var};
use poem::{EndpointExt, Route, Server, listener::TcpListener, middleware::Cors};
use poem_openapi::{
    ApiResponse, Object, OpenApi, OpenApiService,
    payload::{Json, PlainText},
};
use sqlx::PgPool;

use crate::chapter_event::handlers::ChapterEventApi;

// #[derive(Debug, Serialize, Deserialize)]
// struct ResponseData {
//     hejsan: u8,
// };
//

#[derive(Object)]
struct Hejsan {
    hej: u64,
}

#[derive(Object)]
struct CoolErrorType {
    a: u32,
    b: i32,
    c: String,
}

#[derive(ApiResponse)]
enum HejsanResponse {
    #[oai(status = 200)]
    Ok(Json<Hejsan>),

    #[oai(status = 500)]
    NotInterested(Json<CoolErrorType>),
}

struct Api;
#[OpenApi]
impl Api {
    #[oai(path = "/", method = "get")]
    async fn index(&self) -> PlainText<&'static str> {
        PlainText("hello world?")
    }

    #[oai(path = "/hej", method = "get")]
    async fn hej(&self) -> Json<Hejsan> {
        Json(Hejsan { hej: 28 })
    }

    #[oai(path = "/hej2", method = "get")]
    async fn hej2(&self) -> HejsanResponse {
        HejsanResponse::NotInterested(Json(CoolErrorType {
            a: 1,
            b: -3 - 3,
            c: "hejsan svejsan världen".into(),
        }))
    }
}

struct Api2;
#[OpenApi]
impl Api2 {
    #[oai(path = "/faksing", method = "get")]
    async fn index(&self) -> PlainText<&'static str> {
        PlainText("wtf2")
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let api_service = OpenApiService::new((Api, ChapterEventApi), "Hello world", "1.0.0")
        .server("http://localhost:3000/api");

    fs::write("api.yaml", api_service.spec_yaml()).unwrap();
    let ui = api_service.swagger_ui();

    let cors = Cors::new();
    let pg_pool = PgPool::connect(var("DATABASE_URL").unwrap().as_str())
        .await
        .unwrap();
    let app = Route::new()
        .nest("/api", api_service.data(pg_pool))
        .nest("/docs", ui)
        .with(cors);

    Server::new(TcpListener::bind("localhost:3000"))
        .run(app)
        .await
        .unwrap();
    return Ok(());
}
