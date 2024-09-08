use actix_web::web;
use crate::internal::api::user::controllers::rest::user::{get_user, create_user, update_user, delete_user};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/user")
            .route("/{id}", web::get().to(get_user))
            .route("/", web::post().to(create_user))
            .route("/{id}", web::put().to(update_user))
            .route("/{id}", web::delete().to(delete_user)),
    );
}
