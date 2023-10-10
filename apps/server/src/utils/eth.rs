use std::{env, ops::Mul};

use crate::{
    constant::{FAUCET_NUMBER, GAS_MULTIPLE, PRIVATE_KEY, RPC},
    error::{create_error, ResponseError, insufficient_account_balance_error, env_error},
};
use axum::http::StatusCode;
use ethers::{
    middleware::{
        gas_oracle::{GasOracle, ProviderOracle},
        MiddlewareBuilder, SignerMiddleware,
    },
    providers::{Middleware, Provider},
    signers::LocalWallet,
    types::{Address, BlockNumber, Eip1559TransactionRequest, TransactionRequest, U256},
};

use k256::{elliptic_curve::sec1::ToEncodedPoint, AffinePoint, SecretKey};
use sha3::{Digest, Keccak256};

pub async fn faucet(to_str: String) -> Result<String, ResponseError> {
    let rpc_result = env::var(RPC);
    let private_key_result = env::var(PRIVATE_KEY);
    let faucet_number_result = env::var(FAUCET_NUMBER);
    let gas_multiple_result = env::var(GAS_MULTIPLE);
    if let (Ok(rpc), Ok(private_key), Ok(faucet_str), Ok(gas_multiple)) = (
        rpc_result,
        private_key_result,
        faucet_number_result,
        gas_multiple_result,
    ) {
        let multiple = U256::from_dec_str(&gas_multiple)?;
        let to_vec = hex::decode(to_str.clone())?;
        let to = Address::from_slice(&to_vec);
        let faucet_number = U256::from_dec_str(&faucet_str)?;

        let provider_result = Provider::try_from(rpc);
        let provider;
        match provider_result {
            Ok(p) => provider = p,
            Err(e) => {
                return Err(create_error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    e.to_string(),
                ))
            }
        }
        let keys = private_key.split(",");
        let mut target_private_option = None;
        let mut target_address_option = None;

        for key in keys {
            let address = get_address_by_private_key(String::from(key))?;
            let h160_address = Address::from_slice(&address);
            let balance_result = provider.get_balance(h160_address, None).await;
            if let Ok(balance) = balance_result {
                if balance.ge(&faucet_number) {
                    target_private_option = Some(String::from(key));
                    target_address_option = Some(h160_address);
                    break;
                }
            }
        }

        if let (Some(private_key), Some(from)) = (target_private_option, target_address_option) {
            let wallet = private_key.parse::<LocalWallet>()?;
            let signer_client_result =
                SignerMiddleware::new_with_provider_chain(provider.clone(), wallet).await;
            let client;
            match signer_client_result {
                Ok(signer) => client = signer,
                Err(e) => {
                    return Err(create_error(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        e.to_string(),
                    ))
                }
            }
            let support_1559 = support_1559().await;
            let oracle = ProviderOracle::new(provider.clone());
            let nonce_manager = provider.nonce_manager(from);
            let nonce = nonce_manager
                .get_transaction_count(from, Some(BlockNumber::Pending.into()))
                .await?;

            let send_result;
            if !support_1559 {
                let fee_result = oracle.fetch().await;
                match fee_result {
                    Ok(fee) => {
                        let tx = TransactionRequest::new()
                            .from(from)
                            .to(to)
                            .value(faucet_number)
                            .gas_price(fee.mul(multiple))
                            .nonce(nonce);
                        send_result = client.send_transaction(tx, None).await;
                    }
                    Err(e) => {
                        return Err(create_error(
                            StatusCode::INTERNAL_SERVER_ERROR,
                            e.to_string(),
                        ))
                    }
                }
            } else {
                let fee_result = oracle.estimate_eip1559_fees().await;
                match fee_result {
                    Ok((max_fee_per_gas, max_priority_fee_per_gas)) => {
                        let tx = Eip1559TransactionRequest::new()
                            .from(from)
                            .to(to)
                            .value(faucet_number)
                            .max_fee_per_gas(max_fee_per_gas.mul(multiple))
                            .max_priority_fee_per_gas(max_priority_fee_per_gas.mul(multiple))
                            .nonce(nonce);
                        send_result = client.send_transaction(tx, None).await;
                    }
                    Err(e) =>   return Err(create_error(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        e.to_string(),
                    )),
                }
            }

            //todo tx add gas and gaslimit
            // let send_result = client.send_transaction(tx, None).await;
            match send_result {
                Ok(tx) => {
                    return Ok(String::from(tx.tx_hash().to_string()));
                }
                Err(e) => return Err(create_error(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
            }
            
        } else {
            return Err(insufficient_account_balance_error());
        }

        // if let Ok(faucet_number) = faucet_number_result {
        //     // Wallet::new(rng)
        // } else {
        //     return Err(Box::new(EnvError));
        // }
    } else {
        return Err(env_error());
    }
}

fn get_address_by_private_key(private_key: String) -> Result<Vec<u8>, ResponseError> {
    let p;
    if private_key.starts_with("0x") {
        p = String::from(&private_key[2..]);
    } else {
        p = private_key
    }
    let private_key_bytes = hex::decode(p)?;
    let mut private_key_array: [u8; 32] = [0; 32];
    private_key_array.copy_from_slice(&private_key_bytes);
    let secret_key = SecretKey::from_slice(&private_key_array)?;

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

    let address = address_vec[12..].to_vec();
    return Ok(address);
}

// pub fn address_to_hex(address: Vec<u8>) -> String {
//     return hex::encode(address);
// }
pub async fn support_1559() -> bool {
    let rpc_result = env::var(RPC);

    if let Ok(rpc) = rpc_result {
        let provider_result = Provider::try_from(rpc);
        if let Ok(provider) = provider_result {
            let block_number_result = provider.get_block_number().await;
            if let Ok(block_number) = block_number_result {
                let lastest_result = provider.get_block(block_number).await;
                if let Ok(Some(latest)) = lastest_result {
                    let base_fee = latest.base_fee_per_gas;
                    match base_fee {
                        Some(_) => return true,
                        None => return false,
                    }
                }
            }
        }
    }
    return false;
}
#[cfg(test)]
mod tests {
    use std::env;

    use ethers::{
        middleware::gas_oracle::{GasOracle, ProviderOracle},
        providers::{Middleware, Provider},
        types::U256,
    };

    use crate::{
        constant::{GAS_MULTIPLE, RPC},
        utils::eth::{get_address_by_private_key, support_1559},
    };

    #[test]
    fn test_address() {
        let vec = get_address_by_private_key(String::from(
            "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80",
        ))
        .unwrap();
        assert_eq!(hex::encode(vec), "f39fd6e51aad88f6f4ce6ab8827279cfffb92266");
    }
    #[test]
    fn test_0x_address() {
        let hex = get_address_by_private_key(String::from(
            "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80",
        ))
        .unwrap();
        assert_eq!(hex::encode(hex), "f39fd6e51aad88f6f4ce6ab8827279cfffb92266");
    }

    #[tokio::test]
    async fn test_gas_price() {
        dotenv::dotenv().ok();
        let rpc = env::var(RPC).unwrap();
        let provider = Provider::try_from(rpc).unwrap();
        let gas_price = provider.get_gas_price().await.unwrap().to_string();
        println!("gas_price:{}", gas_price)
    }

    #[tokio::test]
    async fn test_block() {
        dotenv::dotenv().ok();
        let bool = support_1559().await;
        println!("{}", bool)
    }

    #[tokio::test]
    async fn test_gas_fee() {
        dotenv::dotenv().ok();

        let rpc = env::var(RPC).unwrap();
        let provider = Provider::try_from(rpc).unwrap();
        let oracle = ProviderOracle::new(provider);
        // let bool = support_1559().await;
        let fee_1559 = oracle.estimate_eip1559_fees().await.unwrap();
        println!("1559:{:?}", fee_1559);
        let fee = oracle.fetch().await.unwrap();
        println!("not 1559:{:?}", fee);
    }

    #[tokio::test]
    async fn test_u256() {
        dotenv::dotenv().ok();
        let gas_multiple = env::var(GAS_MULTIPLE).unwrap();
        let multiple = U256::from_dec_str(&gas_multiple).unwrap();
        assert_eq!(gas_multiple, multiple.to_string());
    }
}

// 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
