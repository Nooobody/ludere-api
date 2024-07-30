
use std::time::{SystemTime, UNIX_EPOCH};

use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use lambda_http::Error;
use serde::{Deserialize, Serialize};
use sha2::Sha256;


#[derive(Debug, Serialize, Deserialize)]
struct JWTClaims {
    sub: String,
    exp: u64,
    iat: u64
}

pub fn build_jwt(username: String) -> Result<String, Error> {
    let jwt_secret = "asdfzxcasdfzxcasdfzxcasdfzxcasdfzxcasdfzxcasdfzxcvvvvvvvasdfzxcv".to_string();

    let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_bytes())?;

    let epoch = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

    let claims = JWTClaims {
        sub: username,
        iat: epoch,
        exp: epoch + 60 * 30
    };

    Ok(claims.sign_with_key(&key)?)
}
