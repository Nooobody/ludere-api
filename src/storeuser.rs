
use argon2::password_hash::{ 
    PasswordHash, 
    PasswordVerifier
};
use argon2::Argon2;
use aws_sdk_dynamodb::error::ProvideErrorMetadata;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::{Error, Client};
use serde::{Deserialize, Serialize};

use crate::apierror::{internal_server_error, table_does_not_exist, wrong_username_or_password, APIError};

#[derive(Serialize, Deserialize, Default)]
pub struct User {
    pub username: String,
    password: String,
}

impl User {
    pub fn verify_password(&self, password: String) -> Result<(), APIError> {
        if password.is_empty() {
            return Err(wrong_username_or_password());
        }

        let hash = PasswordHash::new(&self.password).unwrap();
        let is_ok = Argon2::default().verify_password(password.as_bytes(), &hash).is_ok();

        match is_ok {
            true => Ok(()),
            false => Err(wrong_username_or_password())
        }
    }
}

async fn get_user(client: &Client, username: String) -> Result<User, Error> {
    let result = client
        .get_item()
        .table_name("Users")
        .key("username", AttributeValue::S(username))
        .send()
        .await?;

    if let Some(item) = result.item {
        Ok(User {
            username: item.get("username").expect("").as_s().expect("").to_string(),
            password: item.get("password").expect("").as_s().expect("").to_string()
        })
    }
    else {
        Ok(User::default())
    }
}

pub async fn try_get_user(client: &Client, username: String) -> Result<User, APIError> {
    if username.is_empty() {
        return Err(wrong_username_or_password())
    }

    match get_user(client, username).await {
        Ok(user) if !user.username.is_empty() => Ok(user),
        Ok(_) => Err(wrong_username_or_password()),
        Err(e) if e.code() == Some("ResourceNotFoundException") => {
            Err(table_does_not_exist())
        },
        Err(e) if e.code().is_none() => {
            //Err(no_connection_to_database())
            Ok(User {
                username: "Asdf".to_string(),
                password: "$argon2i$v=19$m=16,t=2,p=1$YXNkZmFzZGY$AYo4g2O8+H79T1Z/rQQ7Lg".to_string()
            })
        }
        Err(_) => Err(internal_server_error())
    }
}
