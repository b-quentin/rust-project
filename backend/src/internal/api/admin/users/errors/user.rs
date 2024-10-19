use actix_web::http::StatusCode;
use async_graphql::{Error, ErrorExtensions};
use log::{error, info};
use thiserror::Error;

use super::interface::CustomGraphQLError;

#[derive(Error, Debug)]
pub enum AdminUserAuthError {
    #[error("User not found: {0}")]
    UserNotFound(String),

    #[error("Invalid password")]
    InvalidPassword,

    #[error("Unexpected error: {0}")]
    UnexpectedError(String),
}

impl CustomGraphQLError for AdminUserAuthError {
    fn new(&self) -> Error {
        match &self {
            AdminUserAuthError::UserNotFound(user) => {
                info!("User not found: {}", user);
            }
            AdminUserAuthError::InvalidPassword => {
                info!("Password mismatch");
            }
            AdminUserAuthError::UnexpectedError(msg) => {
                error!("Unexpected error: {}", msg);
            }
        }

        Error::new(match self {
            AdminUserAuthError::UserNotFound(_) => "The requested user does not exist.",
            AdminUserAuthError::InvalidPassword => "Invalid credentials.",
            AdminUserAuthError::UnexpectedError(_) => "An unexpected internal error occurred.",
        })
        .extend_with(|_err, extensions| {
            match self {
                AdminUserAuthError::UserNotFound(_) => {
                    extensions.set("code", StatusCode::NOT_FOUND.as_u16()); // HTTP 404
                    extensions.set("message", "USER_NOT_FOUND");
                }
                AdminUserAuthError::InvalidPassword => {
                    extensions.set("code", StatusCode::UNAUTHORIZED.as_u16()); // HTTP 401
                    extensions.set("message", "INVALID_PASSWORD");
                }
                AdminUserAuthError::UnexpectedError(_) => {
                    extensions.set("code", StatusCode::INTERNAL_SERVER_ERROR.as_u16()); // HTTP 500
                    extensions.set("message", "UNEXPECTED_ERROR");
                }
            }
        })
    }
}
