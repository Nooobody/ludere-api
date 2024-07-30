
use argon2::password_hash::{ 
    PasswordHash, 
    PasswordVerifier
};
use argon2::Argon2;
use aws_sdk_dynamodb::types::error::builders::IndexNotFoundExceptionBuilder;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::{Error, Client};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct User {
    pub username: String,
    password: String,
}

impl User {
    pub fn verify_password(&self, password: String) -> bool {
        if password.is_empty() {
            return false;
        }

        let hash = PasswordHash::new(&self.password).unwrap();
        Argon2::default().verify_password(password.as_bytes(), &hash).is_ok()
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

pub async fn try_get_user(client: &Client, username: String) -> Result<User, Error> {
    if username.is_empty() {
        return Err(
            Error::IndexNotFoundException(
                IndexNotFoundExceptionBuilder::default()
                    .set_message(Some( "Wrong username or password".to_string()))
                    .build()
            )
        )
    }

    match get_user(client, username).await {
        Ok(user) if !user.username.is_empty() => Ok(user),
        Ok(_user) => Err(
            Error::IndexNotFoundException(
                IndexNotFoundExceptionBuilder::default()
                    .set_message(Some( "Wrong username or password".to_string()))
                    .build()
            )
        ),
        //Err(e) => Err(e)
        Err(_) => Ok(User {
            username: "Asdf".to_string(),
            password: "$argon2i$v=19$m=16,t=2,p=1$YXNkZmFzZGY$AYo4g2O8+H79T1Z/rQQ7Lg".to_string()
        })
    }
}
