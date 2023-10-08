use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use crate::service;

#[derive(Serialize)]
pub struct Code {
    code: String,
}

#[derive(Deserialize)]
pub struct VerificationCode {
    hash: String,
    root: String,
    nonce: String,
}
pub async fn get_root_code() -> (StatusCode, Json<Code>) {
    let code_result = service::mail::get_root_code().await;
    match code_result {
        Ok(code) => {
            return (StatusCode::OK, Json(Code { code }));
        }
        Err(e) => return (StatusCode::BAD_REQUEST, Json(Code { code: e })),
    }
}

pub async fn get_verification_code(
    Json(payload): Json<VerificationCode>,
) -> (StatusCode, Json<Code>) {
    let code_result =
        service::mail::get_verification_code(payload.hash, payload.root, payload.nonce).await;
    match code_result {
        Ok(code) => {
            return (StatusCode::OK, Json(Code { code }));
        }
        Err(e) => return (StatusCode::BAD_REQUEST, Json(Code { code: e })),
    }
}
