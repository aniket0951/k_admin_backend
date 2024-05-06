use actix_web::web;

use crate::service::event_service::*;


pub fn event_router() -> actix_web::Scope {
    web::scope("api/event")
        .route("/add-event", web::post().to(add_event))
        .route("/add-file-data/{path}", web::post().to(add_file_data))
        .route("/get-events/{skip}/{limit}", web::get().to(get_events))
        .route("/get-event/{path}", web::get().to(get_event))
        .route("/delete-event/{path}", web::delete().to(delete_event))
        .route("/update-event/{path}", web::put().to(update_event))
        .route("/total-event", web::get().to(total_event))
}