use actix_web::http::StatusCode;
use async_graphql::{Error, ErrorExtensions};
use log::{error, info};
use thiserror::Error;

use super::interface::CustomGraphQLError;

#[derive(Error, Debug)]
pub enum AdminEntityError {
    #[error("Resource not found: {0}")]
    NotFound(String),
}

impl CustomGraphQLError for AdminEntityError {
    fn new(&self) -> Error {
        match &self {
            AdminEntityError::NotFound(resource) => {
                info!("Entity not found: {}", resource);
            }
        }

        Error::new(match self {
            AdminEntityError::NotFound(_) => "The requested resource does not exist.",
        })
        .extend_with(|_err, extensions| {
            match self {
                AdminEntityError::NotFound(_) => {
                    extensions.set("code", StatusCode::NOT_FOUND.as_u16()); // HTTP 404
                    extensions.set("message", "RESOURCE_NOT_FOUND");
                }
            }
        })
    }
}
