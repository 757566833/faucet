use axum::{http::StatusCode, Json};
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

pub async fn faucet(Json(payload): Json<Faucet>) -> (StatusCode, Json<Response>) {
    let address;
    if payload.address.starts_with("0x") {
        address = String::from(&payload.address[2..]);
    } else {
        address = payload.address;
    }
    let hash_result = service::faucet::faucet(address, payload.email, payload.code).await;
    match hash_result {
        Ok(hash) => {
            return (StatusCode::OK, Json(Response { hash }));
        }
        Err(e) => return (StatusCode::BAD_REQUEST, Json(Response { hash: e })),
    }
}
