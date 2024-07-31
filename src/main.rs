
use std::env;

use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::Client;
use axum::{routing::get, extract::Query , Json, Router};
use lambda_http::{run, Error};
//use lambda_http::tracing::{self};

pub mod storeuser;
pub mod jwt;
pub mod apierror;

use serde::Deserialize;
use storeuser::try_get_user;
use jwt::try_build_jwt;
use apierror::APIError;

#[derive(Deserialize)]
struct LoginPayload {
    username: String,
    password: String
}

async fn post_login(Json(payload): Json<LoginPayload>) -> Result<String, APIError> {
    let username = payload.username;
    let password = payload.password;

    verify_login(username, password).await
}

async fn get_login(payload: Query<LoginPayload>) -> Result<String, APIError> {
    let username = payload.0.username;
    let password = payload.0.password;
    verify_login(username, password).await
}

async fn verify_login(username: String, password: String) -> Result<String, APIError> {
    let client = get_dynamodb_client().await;

    let user = try_get_user(&client, username).await?;
    user.verify_password(password)?;

    let token = try_build_jwt(user.username)?;

    Ok(token)
}

async fn get_dynamodb_client() -> Client {
    //Get config from environment.
    let config = aws_config::defaults(BehaviorVersion::latest())
        .test_credentials()
        .load()
        .await;

    let dynamodb_endpoint = env::var("DYNAMODB_ENDPOINT").unwrap_or("http://localhost:8000".to_string());

    let dynamodb_local_config = aws_sdk_dynamodb::config::Builder::from(&config)
        .endpoint_url(dynamodb_endpoint)
        .build();

    //Create the DynamoDB client.
    Client::from_conf(dynamodb_local_config)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // required to enable CloudWatch error logging by the runtime
    //tracing::init_default_subscriber();

    let app = Router::new()
        .route("/login", get(get_login).post(post_login));

    let _ = run(app).await;

    Ok(())
}
