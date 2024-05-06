use actix_web::web;

use crate::service::app_service::*;

pub fn app_router() -> actix_web::Scope {
    web::scope("api/app")
        .route("/add-branch", web::post().to(add_branch))
        .route("/get-branches", web::get().to(get_branches))
        .route("/update-branch/{path}", web::put().to(update_branch))
        .route("/delete-branch/{path}", web::delete().to(delete_branch))
        .route("/get-branch/{path}", web::get().to(get_branch))
}