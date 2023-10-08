use std::env;

use crate::constant::{FAUCET_NUMBER, PRIVATE_KEY, RPC};
use ethereum_types::U256;
use ethers::{
    providers::{Middleware, Provider},
    signers::{LocalWallet, Signer}, types::TransactionRequest,
};

use k256::{elliptic_curve::sec1::ToEncodedPoint, AffinePoint, SecretKey};
use sha3::{Digest, Keccak256};

pub async fn faucet(to:String) -> Result<String, String> {
    let rpc_result = env::var(RPC);
    let private_key_result = env::var(PRIVATE_KEY);
    let faucet_number_result = env::var(FAUCET_NUMBER);
    if let (Ok(rpc), Ok(private_key), Ok(faucet_str)) =
        (rpc_result, private_key_result, faucet_number_result)
    {
        let faucet_number_result = U256::from_dec_str(&faucet_str);
        if let Ok(faucet_number) = faucet_number_result {
            let provider_result = Provider::try_from(rpc);
            let provider;
            match provider_result {
                Ok(p) => provider = p,
                Err(e) => return Err(e.to_string()),
            }
            let keys = private_key.split(",");
            let mut target_option = None;
            for key in keys {
                let address_result = get_address_by_private_key(String::from(key));
                if let Ok(address) = address_result {
                    let balance_result = provider.get_balance(address, None).await;
                    if let Ok(balance) = balance_result {
                        if balance.ge(&faucet_number) {
                            target_option = Some(String::from(key));
                            break;
                        }
                    }
                } else {
                    return Err(String::from("no faucet"));
                }
            }
            if let Some(target) = target_option {
                let wallet_result = target.parse::<LocalWallet>();
                if let Ok(wallet) = wallet_result {
                    let mut client = SignerMiddleware::new(provider, wallet);
                    let tx = TransactionRequest::new()
                        .to(to) // this will use ENS
                        .value(10000)
                        .into();
                    let signature = wallet.sign_transaction(&tx).await.unwrap();
                    provider.send_transaction(signature);
                    return Ok(String::from("value"));
                }
                return Err(String::from("no faucet"));
            } else {
                return Err(String::from("no faucet"));
            }

            // Wallet::new(rng)
        } else {
            return Err(String::from("env error"));
        }
    } else {
        return Err(String::from("env error"));
    }
}

fn get_address_by_private_key(private_key: String) -> Result<String, String> {
    let p;
    if private_key.starts_with("0x") {
        p = String::from(&private_key[2..]);
    } else {
        p = private_key
    }
    let private_key_bytes_result = hex::decode(p);
    let private_key_bytes;
    match private_key_bytes_result {
        Ok(private) => private_key_bytes = private,
        Err(e) => return Err(e.to_string()),
    }
    let mut private_key_array: [u8; 32] = [0; 32];
    private_key_array.copy_from_slice(&private_key_bytes);
    let secret_key_result = SecretKey::from_slice(&private_key_array);
    let secret_key;
    match secret_key_result {
        Ok(secret) => secret_key = secret,
        Err(e) => return Err(e.to_string()),
    }

    // 压缩公钥匙
    // let compress_public_key =  secret_key.public_key().to_sec1_bytes().to_vec();

    // 压缩公钥匙 hex
    // let compress_public_key_hex = hex::encode(public_key.to_sec1_bytes().to_vec());

    let affine_point = AffinePoint::from(secret_key.public_key());
    // 非压缩公钥
    let un_comporess_affine_point = affine_point.to_encoded_point(false).to_bytes();

    // let un_comporess_affine_point_hex = hex::encode(un_comporess_affine_point.clone());

    // println!(" new {:?}", un_comporess_affine_point_hex);
    let mut hasher = Keccak256::new();
    // 去掉开头的 02、03、04
    hasher.update(&un_comporess_affine_point[1..]);
    let address_vec: Vec<u8> = hasher.finalize().to_vec();
    let address = &address_vec[12..];
    return Ok(hex::encode(address));
}

#[cfg(test)]
mod tests {
    use crate::utils::eth::get_address_by_private_key;

    #[tokio::test]
    async fn test_address() {
        let hex = get_address_by_private_key(String::from(
            "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80",
        ))
        .await
        .unwrap();
        assert_eq!(hex, "f39fd6e51aad88f6f4ce6ab8827279cfffb92266");
    }
    #[tokio::test]
    async fn test_0x_address() {
        let hex = get_address_by_private_key(String::from(
            "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80",
        ))
        .await
        .unwrap();
        assert_eq!(hex, "f39fd6e51aad88f6f4ce6ab8827279cfffb92266");
    }
}

// 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
