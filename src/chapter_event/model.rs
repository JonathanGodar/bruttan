use poem_openapi::{Object, types::Type};
use serde::Serialize;
use sqlx::postgres::types::PgInterval;
use time::{OffsetDateTime, Time};
use tokio::time::Interval;
use uuid::Uuid;

#[derive(Debug, Object, Serialize)]
pub struct ChapterEvent {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub total_seats: Option<i32>,
    pub max_tickets_per_payment: Option<i32>,
    pub sales_stop_at: OffsetDateTime,
    pub reservation_duration_seconds: Option<i32>,
    pub event_at: OffsetDateTime,
    pub door_open_before: Option<OffsetDateTime>,
    pub fcfs: bool,
}
