use aes_gcm::{
    aead::{
        consts::{B0, B1},
        generic_array::GenericArray,
        AeadCore, KeyInit, OsRng,
    },
    aes::{
        cipher::typenum::{UInt, UTerm},
        Aes256,
    },
    Aes256Gcm,
    AesGcm, // Or `Aes128Gcm`
};
use dotenv;
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::sync::{Mutex, OnceCell};

mod constant;
mod controller;
mod router;
mod service;
mod utils;

pub static CIPHER_ONCE: OnceCell<AesGcm<Aes256, UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>>> =
    OnceCell::const_new();
pub static NONCE_ONCE: OnceCell<GenericArray<u8, UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>>> =
    OnceCell::const_new();

pub static CODE_MAP: OnceCell<Arc<tokio::sync::Mutex<HashMap<String, String>>>> =
    OnceCell::const_new();

#[tokio::main]
async fn main() {
   
    dotenv::dotenv().ok();
    let key: GenericArray<u8, UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>> =
        Aes256Gcm::generate_key(OsRng);

    let cipher: AesGcm<Aes256, UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>> =
        Aes256Gcm::new(&key);
    let _ = CIPHER_ONCE.set(cipher);

    let nonce: GenericArray<u8, UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>> =
        Aes256Gcm::generate_nonce(&mut OsRng);
    let _ = NONCE_ONCE.set(nonce);

    let _ = CODE_MAP.set(Arc::new(Mutex::new(HashMap::new())));
    // build our application with a route
    let router = router::init_router();

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

#[cfg(test)]
mod tests {
    use std::env;

    #[test]
    fn test_env() {
        // dotenv::from_filename(".env.local").ok();
        dotenv::dotenv().ok();
      
    

        for (key, value) in env::vars() {
            println!("{}: {}", key, value);
        }
    }
}
