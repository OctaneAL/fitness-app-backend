use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use actix_web::http::header;
use dotenv::dotenv;

mod models;
mod schema;
mod auth;
mod handlers;
mod db;

mod stats_handlers;
mod workout_handlers;
mod auth_handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let client = db::connect().await.expect("Failed to connect to database");
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                // .allowed_origin("http://localhost:3000")
                .allow_any_origin()
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                .allowed_header(header::CONTENT_TYPE)
                .supports_credentials()
                .max_age(3600)
            )
            .app_data(web::Data::new(client.clone()))
            .service(handlers::protected_endpoint)
            .service(handlers::get_exercise_catalog)
            .service(handlers::get_muscle_groups)
            .service(handlers::filter_exercises)
            .service(handlers::get_body_regions)
            .service(handlers::get_body_regions_for_muscle_group)
            .service(handlers::get_difficulties)
            .service(handlers::get_equipment)
            .service(handlers::get_muscle_group_id)
            .configure(stats_handlers::init_routes)
            .configure(workout_handlers::init_routes)
            .configure(auth_handlers::init_routes)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
