use actix_web::http::StatusCode;
use async_graphql::{Error, ErrorExtensions};
use log::{error, info};
use thiserror::Error;

use super::interface::CustomGraphQLError;

#[derive(Error, Debug)]
pub enum AdminPermissionError {
    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Resource not found: {0}")]
    NotFound(String),
}

impl CustomGraphQLError for AdminPermissionError {
    fn new(&self) -> Error {
        match &self {
            AdminPermissionError::PermissionDenied(msg) => {
                info!("Permission denied: {}", msg);
            }
            AdminPermissionError::NotFound(resource) => {
                info!("Resource not found: {}", resource);
            }
        }

        Error::new(match self {
            AdminPermissionError::PermissionDenied(_) => "Access denied.",
            AdminPermissionError::NotFound(_) => "The requested resource does not exist.",
        })
        .extend_with(|_err, extensions| {
            match self {
                AdminPermissionError::PermissionDenied(_) => {
                    extensions.set("code", StatusCode::FORBIDDEN.as_u16()); // HTTP 403
                    extensions.set("message", "PERMISSION_DENIED");
                }
                AdminPermissionError::NotFound(_) => {
                    extensions.set("code", StatusCode::NOT_FOUND.as_u16()); // HTTP 404
                    extensions.set("message", "RESOURCE_NOT_FOUND");
                }
            }
        })
    }
}
