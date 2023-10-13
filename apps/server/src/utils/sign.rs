use sha3::{Digest, Keccak256};

fn sign(private: Vec<u8>, rlp: Vec<u8>) -> Vec<u8> {
    let mut hasher = Keccak256::new();
    // 去掉开头的 02、03、04
    hasher.update(&rlp);
    let address_vec: Vec<u8> = hasher.finalize().to_vec();

    let address = address_vec[12..].to_vec();
    return address;
}

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use k256::{
        ecdsa::Signature,
        elliptic_curve::SecretKey,
        schnorr::signature::{DigestSigner, Signer},
    };
    use sha3::{Digest, Keccak256};
    use rlp::RlpStream;
    use crate::utils::{self, rlp::big_num_to_vec};
    #[test]
    fn test_keccak256() {
        let nonce = num_bigint::BigUint::from_str("9").unwrap();
        let max_priority_fee_per_gas = num_bigint::BigUint::from_str("1000000000").unwrap();
        let max_fee_per_gas = num_bigint::BigUint::from_str("1000000007").unwrap();
        let gas_limit = num_bigint::BigUint::from_str("21000").unwrap();
        let to = hex::decode("A303721F08B85af1Fdf7C57152b9e31D4BCa397B").unwrap();
        let value = num_bigint::BigUint::from_str("5000000000000000000").unwrap();
        let rlp_encoded = utils::rlp::encode(utils::rlp::RlpTransactionRequest::Eip1559(
            utils::rlp::RlpEip1559transaction {
                to,
                nonce,
                gas_limit,
                max_fee_per_gas,
                max_priority_fee_per_gas,
                value,
            },
        ));
        let mut hasher = Keccak256::new();
        hasher.update(&rlp_encoded);
        let address_vec: Vec<u8> = hasher.finalize().to_vec();

        assert_eq!(
            "7786a6ad2ffc8e7c6fa8b678b6d27a4575f4ea81873388720a4d43da4b605c8a",
            hex::encode(address_vec)
        )
        // 默认b7+
    }
    #[test]
    fn test_secp256k1() {
        let nonce = num_bigint::BigUint::from_str("9").unwrap();
        let max_priority_fee_per_gas = num_bigint::BigUint::from_str("1000000000").unwrap();
        let max_fee_per_gas = num_bigint::BigUint::from_str("1000000007").unwrap();
        let gas_limit = num_bigint::BigUint::from_str("21000").unwrap();
        let to = hex::decode("A303721F08B85af1Fdf7C57152b9e31D4BCa397B").unwrap();
        let value = num_bigint::BigUint::from_str("5000000000000000000").unwrap();
        let rlp_encoded = utils::rlp::encode(utils::rlp::RlpTransactionRequest::Eip1559(
            utils::rlp::RlpEip1559transaction {
                to:to.clone(),
                nonce:nonce.clone(),
                gas_limit:gas_limit.clone(),
                max_fee_per_gas:max_fee_per_gas.clone(),
                max_priority_fee_per_gas:max_priority_fee_per_gas.clone(),
                value:value.clone(),
            },
        ));
        let mut hasher = Keccak256::new();
        hasher.update(&rlp_encoded);
        let address_vec: Vec<u8> = hasher.clone().finalize().to_vec();
        assert_eq!(
            "7786a6ad2ffc8e7c6fa8b678b6d27a4575f4ea81873388720a4d43da4b605c8a",
            hex::encode(address_vec.clone())
        );
        let private_key_bytes = hex::decode(
            "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80".to_string(),
        )
        .unwrap();
        let mut private_key_array: [u8; 32] = [0; 32];
        private_key_array.copy_from_slice(&private_key_bytes);
        // let secret_key: SecretKey<k256::Secp256k1> = SecretKey::from_slice(&private_key_array).unwrap();

        let sign = k256::ecdsa::SigningKey::from_slice(&private_key_array).unwrap();

        // let hash = hasher.clone().finalize();
        // Digest::
        let (result, id) = sign.sign_digest_recoverable(hasher).unwrap();
        // result.
        // result

        println!("{:?}", hex::encode(result.r().to_bytes().to_vec()));
        println!("{:?}", hex::encode(result.s().to_bytes().to_vec()));
        println!("{:?}", id);
        println!("{:?}", hex::encode(result.to_vec()));

        let mut rlp_stream = RlpStream::new();
        rlp_stream.begin_list(12);
        rlp_stream.append_empty_data();
        rlp_stream.append(&big_num_to_vec(nonce));
        rlp_stream.append(&big_num_to_vec(
            max_priority_fee_per_gas,
        ));
        rlp_stream.append(&big_num_to_vec(max_fee_per_gas));
        rlp_stream.append(&big_num_to_vec(gas_limit));
        rlp_stream.append(&to);
        rlp_stream.append(&big_num_to_vec(value));
        // rlp_stream.append(&[].to_vec());
        rlp_stream.append_empty_data();
        let e: Vec<u8> = vec![];
        rlp_stream.append_list(&e);
        // Get the RLP-encoded bytes
        rlp_stream.append(&to);
        rlp_stream.append(&to);
        rlp_stream.append(&to);
        let rlp_encoded = rlp_stream.as_raw();
        let mut resut = rlp_encoded.to_owned();
        resut.splice(..0, vec![0x02]);
        // 默认b7+
        // 0x02f8728009843b9aca00843b9aca0782520894a303721f08b85af1fdf7c57152b9e31d4bca397b884563918244f4000080c080a0177940d29ad84fc6a11a9d38f93d108c08364404e8498ed61ad68c3651782074a02b5ee943d3507640a969ba2e71988678e86e4bd1f37afecfaca25cc8983cc340
    }
}
