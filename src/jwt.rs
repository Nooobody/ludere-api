
use std::time::{SystemTime, UNIX_EPOCH};

use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey, error::Error};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

use crate::apierror::{access_denied, internal_server_error, APIError};


#[derive(Debug, Serialize, Deserialize)]
pub struct JWTClaims {
    sub: String,
    exp: u64,
    iat: u64
}

fn get_jwt_key() -> Result<Hmac<Sha256>, Error> {
    let jwt_secret = "asdfzxcasdfzxcasdfzxcasdfzxcasdfzxcasdfzxcasdfzxcvvvvvvvasdfzxcv".to_string();

    Ok(Hmac::new_from_slice(jwt_secret.as_bytes())?)
}

fn build_jwt(username: String) -> Result<String, Error> {
    let key = get_jwt_key()?;

    let epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    let claims = JWTClaims {
        sub: username,
        iat: epoch,
        exp: epoch + 60 * 30
    };

    claims.sign_with_key(&key)
}

pub fn try_build_jwt(username: String) -> Result<String, APIError> {
    build_jwt(username).map_err(|_| internal_server_error())
}

fn verify_jwt(token: String) -> Result<JWTClaims, Error> {
    let key = get_jwt_key()?;

    token.verify_with_key(&key)
}

pub fn try_verify_jwt(token: String) -> Result<JWTClaims, APIError> {
    verify_jwt(token).map_err(|_| access_denied())
}
