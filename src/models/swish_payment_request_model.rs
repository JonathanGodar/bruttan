use anyhow::Result;
use sqlx::{
    PgPool,
    prelude::FromRow,
    types::{Uuid, time::PrimitiveDateTime},
};

use crate::services::SwishPaymentRequestStatus;

// #[derive(FromRow)]
#[derive(sqlx::Type, Debug)]
#[sqlx(rename_all = "snake_case", type_name = "swish_payment_request_status")]
pub enum SwishPaymentRequestModelStatus {
    InitializationFailed,
    Pending,
    Paid,
    Declined,
    Cancelled,
    Timedout,
}

#[derive(FromRow)]
pub struct SwishPaymentRequestModel {
    pub id: Uuid,
    pub created_at: PrimitiveDateTime,
    pub amount: i32,
    pub token: Option<String>,
    pub swish_api_response: Option<String>,
    pub status: SwishPaymentRequestModelStatus,
}

impl SwishPaymentRequestModel {
    pub async fn create(db: &PgPool, amount: i32) -> Result<Uuid> {
        let inserted = sqlx::query!(
            "INSERT INTO swish_payment_requests (amount) VALUES ($1) RETURNING (id)",
            amount
        )
        .fetch_one(db)
        .await?
        .id;

        Ok(inserted)
    }

    pub async fn get(db: &PgPool, id: Uuid) -> Result<SwishPaymentRequestModel> {
        let a = sqlx::query_as!(
            SwishPaymentRequestModel,
            r#"SELECT id, created_at, amount, token, swish_api_response, status AS "status: SwishPaymentRequestModelStatus" FROM swish_payment_requests WHERE id = $1"#,
            id
        ).fetch_one(db).await?;

        Ok(a)
    }

    pub async fn set_status(
        db: &PgPool,
        id: Uuid,
        status: SwishPaymentRequestModelStatus,
    ) -> Result<()> {
        sqlx::query!(
            r#"UPDATE swish_payment_requests SET status = ($1) WHERE id = ($2)"#,
            status as SwishPaymentRequestModelStatus,
            id
        )
        .execute(db)
        .await?;

        Ok(())
    }
}
