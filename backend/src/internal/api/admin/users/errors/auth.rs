use actix_web::http::StatusCode;
use async_graphql::{Error, ErrorExtensions};
use log::{error, info};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AdminAuthError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sea_orm::DbErr),
    
    #[error("JWT error: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),
    
    #[error("Password mismatch")]
    InvalidPassword,

    #[error("User not found: {0}")]
    UserNotFound(String),

    #[error("Unexpected error: {0}")]
    UnexpectedError(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Token expired")]
    TokenExpired,
}


impl AdminAuthError {
    pub fn extend(self) -> Error {
        // Log the detailed error using the `error!` macro
        match &self {
            AdminAuthError::DatabaseError(e) => {
                error!("Database error: {:?}", e);
            }
            AdminAuthError::JwtError(e) => {
                error!("JWT error: {:?}", e);
            }
            AdminAuthError::InvalidPassword => {
                info!("Password mismatch");
            }
            AdminAuthError::UserNotFound(user) => {
                info!("User not found: {}", user);
            }
            AdminAuthError::UnexpectedError(msg) => {
                error!("Unexpected error: {}", msg);
            }
            AdminAuthError::PermissionDenied(msg) => {
                info!("Permission denied: {}", msg);
            }
            AdminAuthError::NotFound(resource) => {
                info!("Resource not found: {}", resource);
            }
            AdminAuthError::TokenExpired => {
                info!("Token expired");
            }
        }

        // Return a generic, secure message to the user with HTTP status codes
        Error::new(match self {
            AdminAuthError::DatabaseError(_) => "An internal error occurred while accessing the database.",
            AdminAuthError::JwtError(_) => "An internal error occurred during token validation.",
            AdminAuthError::InvalidPassword => "Invalid credentials.",
            AdminAuthError::UserNotFound(_) => "The requested user does not exist.",
            AdminAuthError::UnexpectedError(_) => "An unexpected internal error occurred.",
            AdminAuthError::PermissionDenied(_) => "Access denied.",
            AdminAuthError::NotFound(_) => "The requested resource does not exist.",
            AdminAuthError::TokenExpired => "The authentication token has expired.",
        })
        .extend_with(|_err, extensions| {
            match self {
                AdminAuthError::DatabaseError(_) => {
                    extensions.set("code", StatusCode::INTERNAL_SERVER_ERROR.as_u16()); // HTTP 500
                    extensions.set("message", "DATABASE_ERROR");
                }
                AdminAuthError::JwtError(_) => {
                    extensions.set("code", StatusCode::UNAUTHORIZED.as_u16()); // HTTP 401
                    extensions.set("message", "JWT_ERROR");
                }
                AdminAuthError::InvalidPassword => {
                    extensions.set("code", StatusCode::UNAUTHORIZED.as_u16()); // HTTP 401
                    extensions.set("message", "INVALID_PASSWORD");
                }
                AdminAuthError::UserNotFound(_) => {
                    extensions.set("code", StatusCode::NOT_FOUND.as_u16()); // HTTP 404
                    extensions.set("message", "USER_NOT_FOUND");
                }
                AdminAuthError::UnexpectedError(_) => {
                    extensions.set("code", StatusCode::INTERNAL_SERVER_ERROR.as_u16()); // HTTP 500
                    extensions.set("message", "UNEXPECTED_ERROR");
                }
                AdminAuthError::PermissionDenied(_) => {
                    extensions.set("code", StatusCode::FORBIDDEN.as_u16()); // HTTP 403
                    extensions.set("message", "PERMISSION_DENIED");
                }
                AdminAuthError::NotFound(_) => {
                    extensions.set("code", StatusCode::NOT_FOUND.as_u16()); // HTTP 404
                    extensions.set("message", "RESOURCE_NOT_FOUND");
                }
                AdminAuthError::TokenExpired => {
                    extensions.set("code", StatusCode::UNAUTHORIZED.as_u16()); // HTTP 401
                    extensions.set("message", "TOKEN_EXPIRED");
                }
            }
        })
    }
}
