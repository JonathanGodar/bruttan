use poem::web::Data;
use poem_openapi::{OpenApi, param::Path, payload::Json};
use sqlx::{PgPool, query_as};
use uuid::Uuid;

use crate::chapter_event::model::ChapterEvent;

pub struct ChapterEventApi;

#[OpenApi]
impl ChapterEventApi {
    #[oai(path = "/chapter_event/", method = "get")]
    async fn get_chapter_events(&self, Data(pg): Data<&PgPool>) -> Json<Vec<ChapterEvent>> {
        let chapter_events = query_as!(ChapterEvent, "select * from chapter_event;")
            .fetch_all(pg)
            .await
            .unwrap();

        Json(chapter_events)
    }

    #[oai(path = "/chapter_event/:chapter_event_id", method = "get")]
    async fn get_chapter_event(
        &self,
        Path(chapter_event_id): Path<Uuid>,
        Data(pg): Data<&PgPool>,
    ) -> Json<ChapterEvent> {
        let chapter_event = query_as!(
            ChapterEvent,
            "select * from chapter_event where id = $1;",
            chapter_event_id
        )
        .fetch_one(pg)
        .await
        .unwrap();

        Json(chapter_event)
    }
}
