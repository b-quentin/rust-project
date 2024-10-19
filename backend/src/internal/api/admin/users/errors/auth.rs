use actix_web::http::StatusCode;
use async_graphql::{Error, ErrorExtensions};
use jsonwebtoken::errors::ErrorKind;
use log::{error, info};
use thiserror::Error;

use super::interface::CustomGraphQLError;

#[derive(Error, Debug)]
pub enum AuthTokenError {
    #[error("JWT error: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),
    
    #[error("Token expired")]
    TokenExpired,
    
    #[error("Invalid token")]
    InvalidToken,
}

impl CustomGraphQLError for AuthTokenError {
    fn new(&self) -> Error {
        match &self {
            AuthTokenError::JwtError(e) => {
                if let ErrorKind::ExpiredSignature = e.kind() {
                    info!("Token expired due to expired signature");
                    return Error::new("The authentication token has expired.")
                        .extend_with(|_err, extensions| {
                            extensions.set("code", StatusCode::UNAUTHORIZED.as_u16()); // HTTP 401
                            extensions.set("message", "TOKEN_EXPIRED");
                        });
                }

                error!("JWT error: {:?}", e);
            }
            AuthTokenError::TokenExpired => {
                info!("Token expired");
            }
            AuthTokenError::InvalidToken => {
                info!("Invalid token");
            }
        }

        Error::new(match self {
            AuthTokenError::JwtError(_) => "An internal error occurred during token validation.",
            AuthTokenError::TokenExpired => "The authentication token has expired.",
            AuthTokenError::InvalidToken => "The token provided is invalid.",
        })
        .extend_with(|_err, extensions| {
            match self {
                AuthTokenError::JwtError(_) => {
                    extensions.set("code", StatusCode::UNAUTHORIZED.as_u16()); // HTTP 401
                    extensions.set("message", "JWT_ERROR");
                }
                AuthTokenError::TokenExpired => {
                    extensions.set("code", StatusCode::UNAUTHORIZED.as_u16()); // HTTP 401
                    extensions.set("message", "TOKEN_EXPIRED");
                }
                AuthTokenError::InvalidToken => {
                    extensions.set("code", StatusCode::UNAUTHORIZED.as_u16()); // HTTP 401
                    extensions.set("message", "INVALID_TOKEN");
                }
            }
        })
    }
}

