#[derive(Deserialize)]
pub struct Faucet {
    address: String,
    email: String,
    code: String,
}
pub async fn send_verification_code(
    Json(payload): Json<Faucet>,
) -> (StatusCode, Json<Code>) {
    let code_result = service::mail::send_verification_code(
        payload.hash,
        payload.root,
        payload.nonce,
        payload.email,
    )
    .await;
    match code_result {
        Ok(code) => {
            return (StatusCode::OK, Json(Code { code }));
        }
        Err(e) => return (StatusCode::BAD_REQUEST, Json(Code { code: e })),
    }
}
