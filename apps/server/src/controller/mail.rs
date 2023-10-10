use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::service;

#[derive(Serialize)]
pub struct Code {
    code: String,
}
#[derive(Deserialize)]
pub struct RootCode {
    email: String,
}

#[derive(Deserialize)]
pub struct VerificationCode {
    hash: String,
    root: String,
    nonce: String,
    email: String,
}
pub async fn get_root_code(
    Json(payload): Json<RootCode>,
) -> (StatusCode, Json<Value>) {
    let code_result = service::mail::get_root_code(payload.email).await;
    match code_result {
        Ok(code) => {
            return (
                StatusCode::OK,
                Json(serde_json::json!( {
                   "code":code
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

pub async fn send_verification_code(
    Json(payload): Json<VerificationCode>,
) -> (StatusCode, Json<Value>) {
    let code_result = service::mail::send_verification_code(
        payload.hash,
        payload.root,
        payload.nonce,
        payload.email,
    )
    .await;
    match code_result {
        Ok(tx) => {
            return (
                StatusCode::OK,
                Json(serde_json::json!( {
                   "book":tx
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
