use actix_web::{web, HttpResponse, Responder};
use crate::db;
use crate::auth;
use crate::models::{
    LoginUser, RegisterUser, LoginResponse
};

async fn login_user(pool: web::Data<db::DbPool>, user: web::Json<LoginUser>) -> impl Responder {
    let client = pool.lock().await;

    let stmt = client.prepare("SELECT id, username, password FROM users WHERE username = $1").await.expect("Error preparing statement");
    let rows = client.query(&stmt, &[&user.username]).await.expect("Error executing query");

    if rows.is_empty() {
        return HttpResponse::Unauthorized().body("Invalid username or password");
    }

    let row = &rows[0];
    let db_password: &str = row.get("password");
    let user_id: i32 = row.get("id");

    if auth::verify_password(&db_password, &user.password) {
        let token = auth::create_jwt(&user.username);
        
        let response = LoginResponse { token, user_id };

        HttpResponse::Ok().json(response)
    } else {
        HttpResponse::Unauthorized().body("Invalid username or password")
    }
}

async fn register_user(pool: web::Data<db::DbPool>, user: web::Json<RegisterUser>) -> impl Responder {
    let client = pool.lock().await;

    let stmt = client
        .prepare("SELECT COUNT(*) FROM users WHERE username = $1")
        .await
        .expect("Error preparing statement");
    let count: i64 = client
        .query_one(&stmt, &[&user.username])
        .await
        .expect("Error executing query")
        .get(0);

    if count > 0 {
        return HttpResponse::Conflict().body("Username already exists");
    }

    if user.password.len() < 8 {
        return HttpResponse::BadRequest().body("Password must be at least 8 characters long");
    }

    let hashed_password = auth::hash_password(&user.password);

    // let hashed_password = hash(&user.password, DEFAULT_COST).expect("Error hashing password");

    let stmt = client.prepare("INSERT INTO users (username, password) VALUES ($1, $2)").await.expect("Error preparing statement");
    client.execute(&stmt, &[&user.username, &hashed_password]).await.expect("Error executing statement");

    HttpResponse::Ok().json(user.into_inner())
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg
    .service(
        web::resource("/login")
            .route(web::post().to(login_user))
    )
    .service(
        web::resource("/register")
            .route(web::post().to(register_user))
    );    
}
