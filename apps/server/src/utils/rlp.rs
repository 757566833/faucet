// use rlp::{encode, decode};

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



//     result
// }

// #[cfg(test)]
// mod tests {

//     // use crate::utils::rlp::{big_num_to_vec, rlp_encode_list, strip_zeros};

//     // use super::rlp_encode;

//     // #[test]
//     // fn test_rlp() {
//     //     let data = vec![1, 2, 3];
//     //     let encode = rlp_encode(&data);
//     //     println!("{}", hex::encode(encode))
//     // }


//     use std::str::FromStr;

//     use crate::utils::rlp::{
//         encode, RlpEip1559transaction, RlpLegacyTransaction, RlpTransactionRequest,
//     };

//     //     println!("{:?}", strip_zeros(&encode))
//     // }
//     #[test]
//     fn test_eip1559() {
//         let chain_id = num_bigint::BigUint::from_str("79653").unwrap();
//         let nonce = num_bigint::BigUint::from_str("10").unwrap();
//         let max_priority_fee_per_gas = num_bigint::BigUint::from_str("1000000000").unwrap();
//         let max_fee_per_gas = num_bigint::BigUint::from_str("1000000007").unwrap();
//         let gas_limit = num_bigint::BigUint::from_str("21000").unwrap();
//         let to = hex::decode("A303721F08B85af1Fdf7C57152b9e31D4BCa397B").unwrap();
//         let value = num_bigint::BigUint::from_str("5000000000000000000").unwrap();
//         let rlp_encoded = encode(RlpTransactionRequest::Eip1559(RlpEip1559transaction {
//             chain_id,
//             to,
//             nonce,
//             gas_limit,
//             max_fee_per_gas,
//             max_priority_fee_per_gas,
//             value,
//         }));
//         // println!("{}",hex::encode(rlp_encoded));
//         assert_eq!("02f2830137250a843b9aca00843b9aca0782520894a303721f08b85af1fdf7c57152b9e31d4bca397b884563918244f4000080c0",hex::encode(rlp_encoded))
//     }
//     #[test]
//     fn test_legacy() {
//         let chain_id = num_bigint::BigUint::from_str("79653").unwrap();
//         let nonce = num_bigint::BigUint::from_str("10").unwrap();
//         let gas_price = num_bigint::BigUint::from_str("1000000007").unwrap();
//         let gas_limit = num_bigint::BigUint::from_str("21000").unwrap();
//         let to = hex::decode("A303721F08B85af1Fdf7C57152b9e31D4BCa397B").unwrap();
//         let value = num_bigint::BigUint::from_str("5000000000000000000").unwrap();
//         let rlp_encoded = encode(RlpTransactionRequest::Legacy(RlpLegacyTransaction {
//             chain_id,
//             to,
//             nonce,
//             gas_limit,
//             gas_price,
//             value,
//         }));
//         // println!("{}",hex::encode(rlp_encoded));
//         assert_eq!("ee0a843b9aca0782520894a303721f08b85af1fdf7c57152b9e31d4bca397b884563918244f4000080830137258080",hex::encode(rlp_encoded))

//         // 默认b7+
//     }
// }
