use actix_web::web::{self};

use crate::service::student_service::*;


pub fn student_router() -> actix_web::Scope {
    web::scope("api/student")
        .route("/add-student", web::post().to(add_student))
        .route("/upload-profile/{path}", web::post().to(upload_profile))
        .route("/get-students/{skip}/{limit}/{level}", web::get().to(get_students))
        .route("/delete-student/{path}", web::delete().to(delete_student))
        .route("/add-parent", web::post().to(add_parent))
        .route("/total_students", web::get().to(total_students))
        .route("/get-student/{path}", web::get().to(get_student))
        .route("/update-student/{path}", web::put().to(update_student))
        .route("/pending-registration", web::get().to(get_pending_registration))
        .route("/login/{path}", web::get().to(studnent_login))

}

// pub fn student_routes(cfg: &mut web::ServiceConfig) {
//     cfg
//         .service(web::scope("/api")
//         .route("/student", web::post().to(add_student)));
// }