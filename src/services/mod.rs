use std::path::Path;

use anyhow::Context;
use anyhow::anyhow;
use reqwest::StatusCode;
use reqwest::{Client, Identity, Url};
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

pub struct Swish {
    client: reqwest::Client,
    base_url: Url,
    callback_url: Url,
    payee_alias: String,
}

pub enum SwishPaymentRequestStatus {
    Paid,
    Pending,
    Cancelled,
    TimedOut,
    Error,
}

pub struct SwishPaymentRequest {
    id: Uuid,
    status: SwishPaymentRequestStatus,
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

    pub async fn create_swish_payment(
        self,
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
        dbg!(&self.base_url);
        // let url = self.base_url + &format!("/api/v2/paymentrequests/{}", model.id.simple())?;
        let url = self.base_url.join(&format!(
            "api/v2/paymentrequests/{}",
            model_id.simple().to_string().to_uppercase()
        ))?;
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

        let model = SwishPaymentRequestModel::get(db, model_id).await?;
        Ok(model.into())
    }
}
