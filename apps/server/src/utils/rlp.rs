// use rlp::{encode, decode};
fn strip_zeros(value: &str) -> Vec<u8> {
    let mut hex = value.to_string();
    if hex.len() % 2 != 0 {
        hex = format!("0{}", hex)
    }

    let mut result = hex::decode(hex).unwrap_or(Vec::new());

    if result.is_empty() {
        return result;
    }

    // Find the first non-zero entry
    let mut start = 0;
    while start < result.len() && result[start] == 0 {
        start += 1;
    }

    // If we started with zeros, strip them
    if start > 0 {
        result.drain(0..start);
    }

    result
}
// fn rlp_encode(data: &Vec<u8>) -> Vec<u8> {
//     let mut result = Vec::new();

//     if data.len() == 1 && data[0] < 128 {
//         // For single bytes less than 128, RLP encoding is the same as the byte itself
//         result.push(data[0]);
//     } else if data.len() <= 55 {
//         // For short strings (up to 55 bytes), RLP encoding starts with 0x80 + length
//         result.push(0x80 + data.len() as u8);
//         result.extend_from_slice(data);
//     } else {
//         // For longer strings, RLP encoding starts with 0xb7 + length_length and then length bytes
//         let length = data.len();
//         let length_bytes = length.to_be_bytes();
//         let length_length = length_bytes.iter().position(|&x| x != 0).unwrap_or(0);
//         result.push(0xb7 + length_length as u8);
//         result.extend_from_slice(&length_bytes[(4 - length_length)..]);
//         result.extend_from_slice(data);
//     }

//     result
// }
pub fn big_num_to_vec(bg: num_bigint::BigUint) -> Vec<u8> {
    return strip_zeros(&bg.to_str_radix(16));
}

// fn rlp_encode_list(data: &Vec<u8>) -> Vec<u8> {
//     let mut result = Vec::new();

//     if data.is_empty() {
//         // Handle empty list
//         result.push(0xc0);
//     } else if data.len() == 1 && data[0] < 128 {
//         // For single bytes less than 128, RLP encoding is the same as the byte itself
//         result.push(data[0]);
//     } else if data.len() <= 55 {
//         // For short strings (up to 55 bytes), RLP encoding starts with 0x80 + length
//         result.push(0x80 + data.len() as u8);
//         result.extend_from_slice(data);
//     } else {
//         // For longer strings, RLP encoding starts with 0xb7 + length_length and then length bytes
//         let length = data.len();
//         let length_bytes = length.to_be_bytes();
//         let length_length = length_bytes.iter().position(|&x| x != 0).unwrap_or(0);
//         result.push(0xb7 + length_length as u8);
//         result.extend_from_slice(&length_bytes[(4 - length_length)..]);
//         result.extend_from_slice(data);
//     }

use rlp::RlpStream;

#[derive(Debug)]
pub struct RlpLegacyTransaction {
    pub to: Vec<u8>,
    pub gas_price: num_bigint::BigUint,
    pub value: num_bigint::BigUint,
    pub nonce: num_bigint::BigUint,
    pub gas_limit: num_bigint::BigUint,
}
#[derive(Debug)]
pub struct RlpEip1559transaction {
    pub to: Vec<u8>,
    pub max_fee_per_gas: num_bigint::BigUint,
    pub max_priority_fee_per_gas: num_bigint::BigUint,
    pub value: num_bigint::BigUint,
    pub nonce: num_bigint::BigUint,
    pub gas_limit: num_bigint::BigUint,
}
pub enum RlpTransactionRequest {
    Legacy(RlpLegacyTransaction),
    Eip1559(RlpEip1559transaction),
}


//     result
// }
pub fn encode(request: RlpTransactionRequest) -> Vec<u8> {
    match request {
        RlpTransactionRequest::Legacy(legacy_transaction) => {
            let mut rlp_stream = RlpStream::new();
            rlp_stream.begin_list(6);
            rlp_stream.append(&big_num_to_vec(legacy_transaction.nonce));
            rlp_stream.append(&big_num_to_vec(legacy_transaction.gas_price));
            rlp_stream.append(&big_num_to_vec(legacy_transaction.gas_limit));
            rlp_stream.append(&legacy_transaction.to);
            rlp_stream.append(&big_num_to_vec(legacy_transaction.value));
            // rlp_stream.append(&[].to_vec());
            rlp_stream.append_empty_data();
            // println!("{:?}",&rlp_stream.out());
            let rlp_encoded = rlp_stream.as_raw();
            
            return rlp_encoded.to_owned();
        }
        RlpTransactionRequest::Eip1559(eip1559_transaction) => {
            // let nonce = big_num_to_vec(num_bigint::BigUint::from_str("9").unwrap());
            // let max_priority_fee_per_gas =
            //     big_num_to_vec(num_bigint::BigUint::from_str("1000000000").unwrap());
            // let max_fee_per_gas = big_num_to_vec(num_bigint::BigUint::from_str("1000000007").unwrap());
            // let gas_limit = big_num_to_vec(num_bigint::BigUint::from_str("21000").unwrap());

            // let to = hex::decode("A303721F08B85af1Fdf7C57152b9e31D4BCa397B").unwrap();
            // let value = big_num_to_vec(num_bigint::BigUint::from_str("5000000000000000000").unwrap());
            // let to = big_num_to_vec(num_bigint::BigUint::from_str("0xA303721F08B85af1Fdf7C57152b9e31D4BCa397B").unwrap());

            let mut rlp_stream = RlpStream::new();
            rlp_stream.begin_list(9);
            rlp_stream.append_empty_data();
            rlp_stream.append(&big_num_to_vec(eip1559_transaction.nonce));
            rlp_stream.append(&big_num_to_vec(
                eip1559_transaction.max_priority_fee_per_gas,
            ));
            rlp_stream.append(&big_num_to_vec(eip1559_transaction.max_fee_per_gas));
            rlp_stream.append(&big_num_to_vec(eip1559_transaction.gas_limit));
            rlp_stream.append(&eip1559_transaction.to);
            rlp_stream.append(&big_num_to_vec(eip1559_transaction.value));
            // rlp_stream.append(&[].to_vec());
            rlp_stream.append_empty_data();
            let e: Vec<u8> = vec![];
            rlp_stream.append_list(&e);
            // Get the RLP-encoded bytes
            let rlp_encoded = rlp_stream.as_raw();

            return rlp_encoded.to_owned();
        }
    }
}

#[cfg(test)]
mod tests {

    // use crate::utils::rlp::{big_num_to_vec, rlp_encode_list, strip_zeros};

    // use super::rlp_encode;

    // #[test]
    // fn test_rlp() {
    //     let data = vec![1, 2, 3];
    //     let encode = rlp_encode(&data);
    //     println!("{}", hex::encode(encode))
    // }
    // #[test]
    // fn test_format() {
    //     // let data = ["1", "2", "3"];
    //     let encode = num_bigint::BigUint::from_str("21000")
    //         .unwrap()
    //         .to_str_radix(16);

    use std::str::FromStr;


    use crate::utils::rlp::{encode, RlpTransactionRequest, RlpEip1559transaction, RlpLegacyTransaction};

    //     println!("{:?}", strip_zeros(&encode))
    // }
    #[test]
    fn test_eip1559() {
        let nonce = num_bigint::BigUint::from_str("9").unwrap();
        let max_priority_fee_per_gas =
        num_bigint::BigUint::from_str("1000000000").unwrap();
        let max_fee_per_gas = num_bigint::BigUint::from_str("1000000007").unwrap();
        let gas_limit = num_bigint::BigUint::from_str("21000").unwrap();
        let to = hex::decode("A303721F08B85af1Fdf7C57152b9e31D4BCa397B").unwrap();
        let value = num_bigint::BigUint::from_str("5000000000000000000").unwrap();
        let rlp_encoded = encode(RlpTransactionRequest::Eip1559(RlpEip1559transaction {
            to,
            nonce,
            gas_limit,
            max_fee_per_gas,
            max_priority_fee_per_gas,
            value,
        }));
        assert_eq!("ef8009843b9aca00843b9aca0782520894a303721f08b85af1fdf7c57152b9e31d4bca397b884563918244f4000080c0",hex::encode(rlp_encoded))
        // 默认b7+
    }
    #[test]
    fn test_legacy() {
        let nonce = num_bigint::BigUint::from_str("9").unwrap();
        let gas_price = num_bigint::BigUint::from_str("1000000000").unwrap();
        let gas_limit = num_bigint::BigUint::from_str("21000").unwrap();
        let to = hex::decode("A303721F08B85af1Fdf7C57152b9e31D4BCa397B").unwrap();
        let value = num_bigint::BigUint::from_str("5000000000000000000").unwrap();
        let rlp_encoded = encode(RlpTransactionRequest::Legacy(RlpLegacyTransaction {
            to,
            nonce,
            gas_limit,
            gas_price,
            value,
        }));
        assert_eq!("e809843b9aca0082520894a303721f08b85af1fdf7c57152b9e31d4bca397b884563918244f4000080",hex::encode(rlp_encoded))

        // 默认b7+
    }
}
