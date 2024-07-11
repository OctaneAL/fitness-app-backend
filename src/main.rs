use actix_web::{web, App, HttpServer};
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
            .app_data(web::Data::new(client.clone()))
            .service(handlers::register_user)
            .service(handlers::login_user)
            .service(handlers::protected_endpoint)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
