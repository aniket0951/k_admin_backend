use actix_web::middleware::Logger;
use actix_web::{web::Data, App, HttpServer};
use actix_cors::Cors;
pub mod models;
pub mod dto;
pub mod repo;
pub mod router;
pub mod service;
pub mod config;
pub mod helper;
pub mod middleware;
use crate::repo::events_repo::EventRepo;
use crate::repo::user_repo::*;
use crate::repo::student_repo::*;
use actix_files as fs;
use crate::router::{event_router::*, user_router::*};
use crate::router::student_routers::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("APP_LOG").is_none() {
        std::env::set_var("APP_LOG", "mahadev=info")
    }
    env_logger::init();

    let db = match  config::db_config::DBConfig::init().await{
        Ok(instance) => instance,
        Err(err) => {
            panic!("{}", err)
        },
    };



    let db_user = UserRepo::init(db.clone()).await;
    let db_user_db = Data::new(db_user);
    let db_student = Data::new(StudentRepo::init(db.clone()));
    let db_event = Data::new(EventRepo::init(db));

    println!("🚀 Server started successfully!");

    HttpServer::new(move || {
        let cors = Cors::default()
        .allow_any_origin()
        .allow_any_method()
        .allow_any_header();
        App::new()
            .wrap(cors)
            .wrap(middleware::auth_middeleware::Authentication)
            .wrap(Logger::default())
            .app_data(db_user_db.clone())
            .app_data(db_student.clone())
            .app_data(db_event.clone())
            .service(event_router())
            .service(student_router())
            .service(user_router())
            .service(fs::Files::new("/static", "./static"))
            
    })
    // .bind(("127.0.0.1", 8000))?
    .bind(("192.168.0.119", 8000))?
    .run()
    .await 

}
