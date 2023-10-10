use axum::{http::StatusCode, Json};
use serde::Deserialize;
use serde_json::Value;

use crate::service;

#[derive(Deserialize)]
pub struct Faucet {
    address: String,
    email: String,
    code: String,
}


pub async fn faucet(Json(payload): Json<Faucet>) -> (StatusCode, Json<Value>) {
    let address;
    if payload.address.starts_with("0x") {
        address = String::from(&payload.address[2..]);
    } else {
        address = payload.address;
    }
    let hash_result = service::faucet::faucet(address, payload.email, payload.code).await;
    match hash_result {
        Ok(tx) => {
            return (
                StatusCode::OK,
                Json(serde_json::json!( {
                   "hash":tx
                })),
            );
        }
        Err(e) => {
            return (
                e.status,
                Json(serde_json::json!({
                    "message": e.message,
                })),
            )
        }
    }
}
