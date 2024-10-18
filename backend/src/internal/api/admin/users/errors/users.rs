use actix_web::http::StatusCode;
use async_graphql::{Error, ErrorExtensions};
use log::error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserAdminError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sea_orm::DbErr),
}


impl UserAdminError {
    pub fn extend(self) -> Error {
        // Log the detailed error using the `error!` macro
        match &self {
            UserAdminError::DatabaseError(e) => {
                error!("Database error: {:?}", e);
            }
        }

        // Return a generic, secure message to the user with HTTP status codes
        Error::new(match self {
            UserAdminError::DatabaseError(_) => "An internal error occurred while accessing the database.",
        })
        .extend_with(|_err, extensions| {
            match self {
                UserAdminError::DatabaseError(_) => {
                    extensions.set("code", StatusCode::INTERNAL_SERVER_ERROR.as_u16()); // HTTP 500
                    extensions.set("message", "DATABASE_ERROR");
                }
            }
        })
    }
}
