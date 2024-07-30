

use std::env;

use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::Client;
use lambda_http::{run, service_fn, tracing::{self}, Body, Error, Request, RequestExt, Response};

pub mod storeuser;
pub mod jwt;

use storeuser::try_get_user;
use jwt::build_jwt;

async fn handle_request(client: &Client, event: Request) -> Result<Response<Body>, Error> {
    let params = event.query_string_parameters();
    let username = params.first("username").unwrap_or_default().to_string();
    let password = params.first("password").unwrap_or_default().to_string();

    let user = match try_get_user(client, username).await {
        Ok(user) => user,
        Err(aws_sdk_dynamodb::Error::Unhandled(_)) => return Ok(build_response(500, "Unexpected server error".to_string())),
        Err(_) => return Ok(build_response(403, "Wrong username or password".to_string()))
    };

    if !user.verify_password(password) {
        return Ok(build_response(403, "Wrong username or password".to_string()));
    }

    let token = build_jwt(user.username)?;

    Ok(build_response(200, token))
}

fn build_response(status_code: u16, message: String) -> Response<Body> {
    Response::builder()
        .status(status_code)
        .header("content-type", "text/html")
        .body(message.into())
        .map_err(Box::new)
        .unwrap()
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    // required to enable CloudWatch error logging by the runtime
    tracing::init_default_subscriber();

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
    let client = Client::from_conf(dynamodb_local_config);

    run(service_fn(|event: Request| async {
        handle_request(&client, event).await
    }))
    .await
}
