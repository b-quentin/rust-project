use actix_web::http::StatusCode;
use async_graphql::{Error, ErrorExtensions};
use log::{error, info};
use thiserror::Error;

use super::interface::CustomGraphQLError;

#[derive(Error, Debug)]
pub enum AdminActionError {
    #[error("Resource not found: {0}")]
    NotFound(String),
}

impl CustomGraphQLError for AdminActionError {
    fn new(&self) -> Error {
        match &self {
            AdminActionError::NotFound(resource) => {
                info!("Action not found: {}", resource);
            }
        }

        Error::new(match self {
            AdminActionError::NotFound(_) => "The requested resource does not exist.",
        })
        .extend_with(|_err, extensions| {
            match self {
                AdminActionError::NotFound(_) => {
                    extensions.set("code", StatusCode::NOT_FOUND.as_u16()); // HTTP 404
                    extensions.set("message", "RESOURCE_NOT_FOUND");
                }
            }
        })
    }
}
