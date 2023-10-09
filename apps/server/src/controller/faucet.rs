use axum::{Json, http::StatusCode};
use serde::{Deserialize, Serialize};

use crate::service;

#[derive(Deserialize)]
pub struct Faucet {
    address: String,
    email: String,
    code: String,
}

#[derive(Serialize)]
pub struct Response {
    hash: String,
}

pub async fn faucet(
    Json(payload): Json<Faucet>,
) -> (StatusCode, Json<Response>) {
    let hash_result = service::faucet::faucet(
        payload.address,
        payload.email,
        payload.code,
    )
    .await;
    match hash_result {
        Ok(hash) => {
            return (StatusCode::OK, Json(Response { hash }));
        }
        Err(e) => return (StatusCode::BAD_REQUEST, Json(Response { hash: e })),
    }
}

