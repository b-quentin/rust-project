use async_graphql::{Error, ErrorExtensions};
use log::error;
use thiserror::Error;
use actix_web::http::StatusCode;

use super::interface::CustomGraphQLError;

#[derive(Error, Debug)]
pub enum AdminDbError {
    #[error("Database error: {0}")]
    DatabaseError(String),
}

impl CustomGraphQLError for AdminDbError {
    fn new(&self) -> Error {
        // Log the detailed error using the `error!` macro
        match &self {
            AdminDbError::DatabaseError(e) => {
                error!("Database error occurred: {:?}", e);
            }
        }

        // Return a secure message to the user with HTTP status codes
        Error::new(match self {
            AdminDbError::DatabaseError(_) => "An internal error occurred while accessing the database.",
        })
        .extend_with(|_err, extensions| {
            match self {
                AdminDbError::DatabaseError(_) => {
                    extensions.set("code", StatusCode::INTERNAL_SERVER_ERROR.as_u16()); // HTTP 500
                    extensions.set("message", "DATABASE_ACCESS_ERROR");
                }
            }
        })
    }
}
