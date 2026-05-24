use std::path::Path;
use std::time::Duration;

use anyhow::Context;
use anyhow::anyhow;
use chrono::prelude::*;
use reqwest::StatusCode;
use reqwest::{Client, Identity, Url};
use serde::Deserialize;
use serde_json::Number;
use serde_json::json;
use sqlx::{PgPool, types::Uuid};
use thiserror::Error;

use crate::models::swish_payment_request_model::SwishPaymentRequestModel;

mod swish_service;

// #[derive(Error, Debug)]
// pub enum SwishError {
//     #[error("Unable to create swish client")]
//     SwishClientCreationError(#[from] reqwest::Error),
//
//     #[error("Negative Payment Amount")]
//     NegativePaymentAmount,
// }
//

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SwishPaymentRequestResponse {
    id: String,
    callback_url: String,
    amount: Number,
    status: String,
    date_created: DateTime<Utc>,
    date_paid: Option<DateTime<Utc>>,
    error_code: Option<String>,
    error_message: Option<String>,
}

pub struct Swish {
    client: reqwest::Client,
    base_url: Url,
    callback_url: Url,
    payee_alias: String,
}

#[derive(Debug)]
pub enum SwishPaymentRequestStatus {
    Paid,
    Pending,
    Cancelled,
    TimedOut,
    Error,
}

#[derive(Debug)]
pub struct SwishPaymentRequest {
    pub id: Uuid,
    pub status: SwishPaymentRequestStatus,
}

impl From<SwishPaymentRequestModel> for SwishPaymentRequest {
    fn from(model: SwishPaymentRequestModel) -> Self {
        Self {
            id: model.id,
            status: SwishPaymentRequestStatus::Pending,
        }
    }
}

impl Swish {
    pub fn new(
        swish_base_url: Url,
        callback_url: Url,
        identity: Identity,
        payee_alias: String,
    ) -> anyhow::Result<Self> {
        let client = Client::builder()
            .identity(identity)
            .build()
            .context("Could not create swish clientt")?;

        Ok(Self {
            client,
            base_url: swish_base_url,
            callback_url,
            payee_alias,
        })
    }

    fn update_swish_payment(pg: &PgPool, req_response: SwishPaymentRequestResponse) {
        todo!()
    }

    fn payment_request_endpoint(&self, id: Uuid) -> anyhow::Result<Url> {
        let a = self.base_url.join(&format!(
            "api/v2/paymentrequests/{}",
            id.simple().to_string().to_uppercase()
        ))?;

        Ok(a)
    }

    pub async fn sync_swish_payment(&self, id: Uuid) -> anyhow::Result<()> {
        dbg!("Syncing");
        let url = self.base_url.join(&format!(
            "api/v1/paymentrequests/{}",
            id.simple().to_string().to_uppercase()
        ))?;

        tokio::time::sleep(Duration::from_secs(5)).await;

        let response = self.client.get(url).send().await?;
        // dbg!(response.text().await);
        // dbg!(response.text().await?);
        let response: SwishPaymentRequestResponse = response.json().await?;
        dbg!(response);
        // .map(|s| serde_json::from_str::<SwishPaymentRequestResponse>(s));
        Ok(())
    }

    pub async fn create_swish_payment(
        &self,
        db: &PgPool,
        amount: i32,
        message: &str,
    ) -> anyhow::Result<SwishPaymentRequest> {
        if amount < 1 {
            return Err(anyhow!("Bad things"));
        }

        let model_id = SwishPaymentRequestModel::create(db, amount).await?;
        let request_body = json!({
            "payeeAlias": "1234679304",
            "amount": 10,
            "callbackUrl": "https://tun1.ngodar.com/api/swish/callback",
            "currency": "SEK",
            "message": message,
        }
        );

        // let request_body = json!({
        //     "payeeAlias": self.payee_alias,
        //     "amount": amount,
        //     "callbackUrl": self.callback_url.as_str(),
        //     "currency": "SEK",
        //     "message": message
        // });
        // let url = self.base_url + &format!("/api/v2/paymentrequests/{}", model.id.simple())?;
        let url = self.payment_request_endpoint(model_id)?;

        // let url = self.base_url.join(&format!(
        //     "api/v2/paymentrequests/{}",
        //     model_id.simple().to_string().to_uppercase()
        // ))?;
        // .join(&format!("api/v2/paymentrequests/{}", 0BE6E5B43EB441B6B54FBD2907C4ACEB))?;
        //
        // TODO Hanlde errors more elegantly
        let response = self.client.put(url).json(&request_body).send().await?;

        if response.status() != StatusCode::CREATED {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(anyhow!(
                "Failed to create swish payment request, Status {}; Body {}",
                status,
                body
            ));
        }

        let payment_request_token = response
            .headers()
            .get("paymentrequesttoken")
            .context("Payment request token header was not set, but payment was created")?
            .to_str()?
            .to_string();

        self.sync_swish_payment(model_id).await?;

        let model = SwishPaymentRequestModel::get(db, model_id).await?;
        Ok(model.into())
    }
}
