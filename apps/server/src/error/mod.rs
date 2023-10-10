use axum::http::StatusCode;

#[derive(Debug)]
pub struct ResponseError {
    pub status: StatusCode,
    pub message: String,
}

// letter 将email 转为地址的错误
impl From<lettre::address::AddressError> for ResponseError {
    fn from(error: lettre::address::AddressError) -> Self {
        ResponseError {
            message: error.to_string(),
            status: StatusCode::BAD_REQUEST,
        }
    }
}

// letter 发送邮件的 参数 builder 错误
impl From<lettre::error::Error> for ResponseError {
    fn from(error: lettre::error::Error) -> Self {
        ResponseError {
            message: error.to_string(),
            status: StatusCode::BAD_REQUEST,
        }
    }
}

// letter smtp 错误
impl From<lettre::transport::smtp::Error> for ResponseError {
    fn from(error: lettre::transport::smtp::Error) -> Self {
        ResponseError {
            message: error.to_string(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
// GAS MULTIPLE 转为 U256 的错误
impl From<ethers::abi::ethereum_types::FromDecStrErr> for ResponseError {
    fn from(error: ethers::abi::ethereum_types::FromDecStrErr) -> Self {
        ResponseError {
            message: error.to_string(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
// 地址hex 转为对象错误
impl From<hex::FromHexError> for ResponseError {
    fn from(error: hex::FromHexError) -> Self {
        ResponseError {
            message: error.to_string(),
            status: StatusCode::BAD_REQUEST,
        }
    }
}

// 私钥转地址失败
impl From<k256::elliptic_curve::Error> for ResponseError {
    fn from(error: k256::elliptic_curve::Error) -> Self {
        ResponseError {
            message: error.to_string(),
            status: StatusCode::BAD_REQUEST,
        }
    }
}
/**
 * 私钥生成钱包失败
 */
impl From<ethers::signers::WalletError> for ResponseError {
    fn from(error: ethers::signers::WalletError) -> Self {
        ResponseError {
            message: error.to_string(),
            status: StatusCode::BAD_REQUEST,
        }
    }
}
/**
 * 链接自定义rpc签名
 */
// impl From<ethers::middleware::signer::SignerMiddlewareError<ethers::providers::Provider<ethers::prelude::Http>,ethers::signers::Wallet<ecdsa::signing::SigningKey<Secp256k1>>>> for ResponseError {
//     fn from(error:ethers::signers::WalletError) -> Self {
//         ResponseError {
//             message: error.to_string(),
//             status: StatusCode::BAD_REQUEST,
//         }
//     }
// }

/**
 * 获取nonce 错误
 */
impl
    From<
        ethers::prelude::nonce_manager::NonceManagerError<
            ethers::providers::Provider<ethers::prelude::Http>,
        >,
    > for ResponseError
{
    fn from(
        error: ethers::prelude::nonce_manager::NonceManagerError<
            ethers::providers::Provider<ethers::prelude::Http>,
        >,
    ) -> Self {
        ResponseError {
            message: error.to_string(),
            status: StatusCode::BAD_REQUEST,
        }
    }
}
// impl From<ethers_middleware::gas_oracle::GasOracleError> for ResponseError {
//     fn from(error:ethers_middleware::gas_oracle::GasOracleError>) -> Self {
//         ResponseError {
//             message: error.to_string(),
//             status: StatusCode::BAD_REQUEST,
//         }
//     }
// }
impl From<aes_gcm::Error> for ResponseError {
    fn from(error: aes_gcm::Error) -> Self {
        ResponseError {
            message: error.to_string(),
            status: StatusCode::BAD_REQUEST,
        }
    }
}

impl From<base64::DecodeError> for ResponseError {
    fn from(error: base64::DecodeError) -> Self {
        ResponseError {
            message: error.to_string(),
            status: StatusCode::BAD_REQUEST,
        }
    }
}
impl From<std::string::FromUtf8Error> for ResponseError {
    fn from(error: std::string::FromUtf8Error) -> Self {
        ResponseError {
            message: error.to_string(),
            status: StatusCode::BAD_REQUEST,
        }
    }
}

impl From<std::num::ParseIntError> for ResponseError {
    fn from(error: std::num::ParseIntError) -> Self {
        ResponseError {
            message: error.to_string(),
            status: StatusCode::BAD_REQUEST,
        }
    }
}
impl From<std::time::SystemTimeError> for ResponseError {
    fn from(error: std::time::SystemTimeError) -> Self {
        ResponseError {
            message: error.to_string(),
            status: StatusCode::BAD_REQUEST,
        }
    }
}
impl From<axum::http::Error> for ResponseError {
    fn from(error: axum::http::Error) -> Self {
        ResponseError {
            message: error.to_string(),
            status: StatusCode::BAD_REQUEST,
        }
    }
}
impl From<hyper::Error> for ResponseError {
    fn from(error: hyper::Error) -> Self {
        ResponseError {
            message: error.to_string(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
impl From<serde_json::Error> for ResponseError {
    fn from(error: serde_json::Error) -> Self {
        ResponseError {
            message: error.to_string(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}


//  邮箱格式错误
pub fn email_format_error() -> ResponseError {
    ResponseError {
        message: String::from("Email format error"),
        status: StatusCode::BAD_REQUEST,
    }
}

//  邮箱格式错误
pub fn env_error() -> ResponseError {
    ResponseError {
        message: String::from("Env is error"),
        status: StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub fn insufficient_account_balance_error() -> ResponseError {
    ResponseError {
        message: String::from("Insufficient account balance"),
        status: StatusCode::INTERNAL_SERVER_ERROR,
    }
}
pub fn aes_cache_error() -> ResponseError {
    ResponseError {
        message: String::from("can't find aes once"),
        status: StatusCode::INTERNAL_SERVER_ERROR,
    }
}
/**
 * 验证码map 缺失 错误 正常不会触发
 */
pub fn code_cache_error() -> ResponseError {
    ResponseError {
        message: String::from("can't find code map"),
        status: StatusCode::INTERNAL_SERVER_ERROR,
    }
}
/**
 * 针对于同一个邮箱 发送有冷却时间
 */
pub fn code_cooling_error() -> ResponseError {
    ResponseError {
        message: String::from("The function of sending verification code is cooling down"),
        status: StatusCode::BAD_REQUEST,
    }
}

/**
 * 水龙头领取后 邮箱的冷却 map 缺失 正常不会触发
 */
pub fn email_cool_cache_error() -> ResponseError {
    ResponseError {
        message: String::from("can't find email send map"),
        status: StatusCode::INTERNAL_SERVER_ERROR,
    }
}

/**
 * 水龙头成功领取 邮箱的冷却 map
 */
pub fn email_cooling_error() -> ResponseError {
    ResponseError {
        message: String::from("Email is cooling down"),
        status: StatusCode::BAD_REQUEST,
    }
}
/**
 * 水龙头领取后 地址的冷却 map 缺失 正常不会触发
 */
pub fn address_cool_cache_error() -> ResponseError {
    ResponseError {
        message: String::from("can't find address send map"),
        status: StatusCode::INTERNAL_SERVER_ERROR,
    }
}

/**
 * 水龙头领取后 地址的冷却
 */
pub fn address_cooling_error() -> ResponseError {
    ResponseError {
        message: String::from("Address is cooling down"),
        status: StatusCode::BAD_REQUEST,
    }
}
/**
 * 前段请求的发送邮箱验证码 hash验证失败
 */
pub fn verify_hash_error() -> ResponseError {
    ResponseError {
        message: String::from("hash is error"),
        status: StatusCode::BAD_REQUEST,
    }
}

pub fn timeout_error() -> ResponseError {
    ResponseError {
        message: String::from("timeout"),
        status: StatusCode::BAD_REQUEST,
    }
}

/**
 * 为幽灵依赖准备
 */
pub fn create_error(status: StatusCode, message: String) -> ResponseError {
    ResponseError { status, message }
}

// /**
//  * aes相关的once cell 缺失 错误 正常不会触发
//  */
// #[derive(Debug)]
// pub struct AesCacheError;

// impl std::error::Error for AesCacheError {}

// impl std::fmt::Display for AesCacheError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "can't find aes once")
//     }
// }

// /**
//  * 包装aes的error
//  */
// #[derive(Debug)]
// pub struct AesAeadError {
//     pub inner: aes_gcm::Error, // 用于包装第三方结构体
// }
// impl From<aes_gcm::Error> for AesAeadError {
//     fn from(inner: aes_gcm::Error) -> Self {
//         AesAeadError { inner }
//     }
// }
// impl std::error::Error for AesAeadError {}

// impl std::fmt::Display for AesAeadError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "Aes Aead error")
//     }
// }
