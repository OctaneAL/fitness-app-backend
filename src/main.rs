use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use actix_web::http::header;
use dotenv::dotenv;

mod models;
mod schema;
mod auth;
mod handlers;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let client = db::connect().await.expect("Failed to connect to database");
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                .allowed_origin("http://localhost:3000")
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                .allowed_header(header::CONTENT_TYPE)
                .supports_credentials()
                .max_age(3600)
            )
            .app_data(web::Data::new(client.clone()))
            .service(handlers::register_user)
            .service(handlers::login_user)
            .service(handlers::protected_endpoint)
            .service(handlers::add_workout)
            .service(handlers::update_workout)
            .service(handlers::delete_workout)
            .service(handlers::get_workouts)
            .service(handlers::get_exercise_catalog)
            .service(handlers::get_user_workouts)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
