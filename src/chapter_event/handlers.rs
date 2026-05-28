use poem::web::Data;
use poem_openapi::{OpenApi, payload::Json};
use sqlx::{PgPool, query_as};

use crate::chapter_event::model::ChapterEvent;

pub struct ChapterEventApi;

#[OpenApi]
impl ChapterEventApi {
    #[oai(path = "/chapter_event/", method = "get")]
    async fn GetChapterEvents(&self, Data(pg): Data<&PgPool>) -> Json<Vec<ChapterEvent>> {
        let a = query_as!(ChapterEvent, "select * from chapter_event;")
            .fetch_all(pg)
            .await
            .unwrap();

        Json(a)
    }
}
