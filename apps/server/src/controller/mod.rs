use serde::Serialize;

pub mod faucet;
pub mod html;
pub mod mail;

#[derive(Serialize)]
pub struct ErrResponse {
    msg: String,
}
