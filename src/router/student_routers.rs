use actix_web::web;

use crate::service::student_service::*;


pub fn student_router() -> actix_web::Scope {
    web::scope("api/student")
        .route("/add-student", web::post().to(add_student))
        .route("/upload-profile", web::post().to(upload_profile))
        .route("/get-students", web::get().to(get_students))
        .route("/delete-student/{path}", web::delete().to(delete_student))
        .route("/add-parent", web::post().to(add_parent))

}

// pub fn student_routes(cfg: &mut web::ServiceConfig) {
//     cfg
//         .service(web::scope("/api")
//         .route("/student", web::post().to(add_student)));
// }