use actix_web::web;

use crate::service::user_service::*;

pub fn user_router() -> actix_web::Scope {
    web::scope("api")
        .route("/user", web::post().to(add_user))
        .route("/users", web::get().to(get_users))
        .route("/user/{path}", web::put().to(update_user))
        .route("/user/{path}", web::delete().to(delete_user))
        .route("/login", web::post().to(login))

}

// pub fn user_router(cfg: &mut web::ServiceConfig) {
//     cfg
//         .service(web::scope("/api")
//             .route("/user", web::post().to(add_user))
//             .route("/users", web::get().to(get_users))
//             .route("/user/{path}", web::put().to(update_user))
//             .route("/user/{path}", web::delete().to(delete_user))
//             .route("/login", web::post().to(login)))
//             .service(student_router())
            
//             ;

//     // Call the routes from the student router directly
//     // student_routes(cfg);
// }