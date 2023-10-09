use std::env;

use crate::constant::{FAUCET_NUMBER, PRIVATE_KEY, RPC};
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
pub async fn faucet(to: String) -> Result<String, String> {
    let rpc_result = env::var(RPC);
    let private_key_result = env::var(PRIVATE_KEY);
    let faucet_number_result = env::var(FAUCET_NUMBER);
    if let (Ok(rpc), Ok(private_key), Ok(faucet_str)) =
        (rpc_result, private_key_result, faucet_number_result)
    {
        let to_result = hex::decode(to);
        let to;
        match to_result {
            Ok(t) => to = Address::from_slice(&t),
            Err(e) => return Err(e.to_string()),
        };
        let faucet_number_result = U256::from_dec_str(&faucet_str);
        if let Ok(faucet_number) = faucet_number_result {
            let provider_result = Provider::try_from(rpc);
            let provider;
            match provider_result {
                Ok(p) => provider = p,
                Err(e) => return Err(e.to_string()),
            }
            let keys = private_key.split(",");
            let mut target_private_option = None;
            let mut target_address_option = None;

            for key in keys {
                let address_result = get_address_by_private_key(String::from(key));
                if let Ok(address) = address_result {
                    let h160_address = Address::from_slice(&address);
                    let balance_result = provider.get_balance(h160_address, None).await;
                    if let Ok(balance) = balance_result {
                        if balance.ge(&faucet_number) {
                            target_private_option = Some(String::from(key));
                            target_address_option = Some(h160_address);
                            break;
                        }
                    }
                } else {
                    return Err(String::from(format!(
                        "private key to address failed {}",
                        key
                    )));
                }
            }

            if let (Some(private_key), Some(from)) = (target_private_option, target_address_option)
            {
                let wallet_result = private_key.parse::<LocalWallet>();
                if let Ok(wallet) = wallet_result {
                    let signer_client_result = SignerMiddleware::new_with_provider_chain(provider.clone(), wallet).await;
                    let client;
                    if let Ok(signer) = signer_client_result  {
                        client = signer
                    }else{
                        return Err(String::from("rpc error"))
                    }
                    let support_1559 = support_1559().await;
                    let oracle = ProviderOracle::new(provider.clone());
                    let nonce_manager = provider.nonce_manager(from);
                    let curr_nonce_result = nonce_manager
                        .get_transaction_count(from, Some(BlockNumber::Pending.into()))
                        .await;
                    let nonce;
                    match curr_nonce_result {
                        Ok(n) => nonce = n,
                        Err(e) => return Err(e.to_string()),
                    }
                    // provider.estimate_eip1559_fees(estimator)
                    // let tx = TransactionRequest::pay(to, faucet_number);
                    let send_result;
                    if !support_1559 {
                        let fee_result = oracle.fetch().await;
                        match fee_result {
                            Ok(fee) => {
                                let tx = TransactionRequest::new()
                                    .from(from)
                                    .to(to)
                                    .value(faucet_number)
                                    .gas_price(fee)
                                    .nonce(nonce);
                                send_result = client.send_transaction(tx, None).await;
                            }
                            Err(e) => return Err(e.to_string()),
                        }
                    } else {
                        let fee_result = oracle.estimate_eip1559_fees().await;
                        match fee_result {
                            Ok((max_fee_per_gas, max_priority_fee_per_gas)) => {
                                let tx = Eip1559TransactionRequest::new()
                                    .from(from)
                                    .to(to)
                                    .value(faucet_number)
                                    .max_fee_per_gas(max_fee_per_gas)
                                    .max_priority_fee_per_gas(max_priority_fee_per_gas)
                                    .nonce(nonce);
                                send_result = client.send_transaction(tx, None).await;
                            }
                            Err(e) => return Err(e.to_string()),
                        }
                    }

                    //todo tx add gas and gaslimit
                    // let send_result = client.send_transaction(tx, None).await;
                    match send_result {
                        Ok(tx) => return Ok(String::from(tx.tx_hash().to_string())),
                        Err(e) => return Err(e.to_string()),
                    }
                    // return Ok(String::from(""))
                }
                return Err(String::from("wallet failed"));
            } else {
                return Err(String::from("no faucet all keys"));
            }

            // Wallet::new(rng)
        } else {
            return Err(String::from("env error"));
        }
    } else {
        return Err(String::from("env error"));
    }
}

fn get_address_by_private_key(private_key: String) -> Result<Vec<u8>, String> {
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
    };

    use crate::{
        constant::RPC,
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
}

// 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
