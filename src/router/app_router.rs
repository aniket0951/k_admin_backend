use actix_web::web;

use crate::service::app_service::*;

pub fn app_router() -> actix_web::Scope {
    web::scope("api/app")
        .route("/add-branch", web::post().to(add_branch))
        .route("/get-branches", web::get().to(get_branches))
        .route("/update-branch/{path}", web::put().to(update_branch))
        .route("/delete-branch/{path}", web::delete().to(delete_branch))
        .route("/get-branch/{path}", web::get().to(get_branch))
        .route("/app-counts", web::get().to(app_counts))

        .route("/guest-access", web::get().to(guest_access_token))

        // fee router
        .route("/add-fee", web::post().to(add_fee))
        .route("/get-fee", web::get().to(get_fee))
        .route("/enable-discount/{path}", web::put().to(make_discount_Active))
        .route("/delete-fee/{path}", web::delete().to(delete_fee))

        // course router
        .route("/add-course", web::route().to(add_course))
        .route("/list-course", web::route().to(list_course))
        .route("/active-course", web::route().to(active_course))
        .route("/update-course", web::route().to(update_course))

}