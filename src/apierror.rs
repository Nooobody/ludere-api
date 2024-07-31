use axum::http::StatusCode;

pub type APIError = (StatusCode, String);

pub fn wrong_username_or_password() -> APIError {
    (StatusCode::FORBIDDEN, "Wrong username or password".to_string())
}

pub fn internal_server_error() -> APIError {
    (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected server error".to_string())
}

pub fn table_does_not_exist() -> APIError {
    (StatusCode::INTERNAL_SERVER_ERROR, "Database table does not exist".to_string())
    //panic!("Database table does not exist");
}

pub fn no_connection_to_database() -> APIError {
    (StatusCode::SERVICE_UNAVAILABLE, "No connection to database".to_string())
}
