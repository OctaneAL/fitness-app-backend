use actix_web::{web, HttpResponse, HttpRequest, Responder, Error, post, get};

use crate::db;
use crate::auth;
use crate::models::{RegisterUser, LoginUser};

// pub fn config(cfg: &mut web::ServiceConfig, pool: &db::DbPool) {
//     cfg.service(
//         web::scope("/api")
//             .service(web::resource("/register").route(web::post().to(|user: web::Json<RegisterUser>| register_user(user))))
//             .service(web::resource("/login").route(web::post().to(|user: web::Json<LoginUser>| login_user(user))))
//             .service(web::resource("/protected").route(web::get().to(protected_endpoint))),
//     );
// }

#[post("/register")]
async fn register_user(pool: web::Data<db::DbPool>, user: web::Json<RegisterUser>) -> impl Responder {
    let client = pool.get_ref();

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

#[post("/login")]
async fn login_user(pool: web::Data<db::DbPool>, user: web::Json<LoginUser>) -> impl Responder {
    let client = pool.get_ref();

    let stmt = client.prepare("SELECT id, username, password FROM users WHERE username = $1").await.expect("Error preparing statement");
    let rows = client.query(&stmt, &[&user.username]).await.expect("Error executing query");

    if rows.is_empty() {
        return HttpResponse::Unauthorized().body("Invalid username or password");
    }

    let row = &rows[0];
    let db_password: &str = row.get("password");

    if auth::verify_password(&db_password, &user.password) {
        let token = auth::create_jwt(&user.username);
        HttpResponse::Ok().json(token)
    } else {
        HttpResponse::Unauthorized().body("Invalid username or password")
    }
    // if verify(&user.password, db_password).expect("Error verifying password") {
    //     let token = create_jwt(&user.username);
    //     HttpResponse::Ok().json(token)
    // } else {
    //     HttpResponse::Unauthorized().body("Invalid username or password")
    // }
}

#[get("/protected")]
async fn protected_endpoint(req: HttpRequest) -> Result<HttpResponse, Error> {
    if let Some(auth_value) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_value.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..]; // Remove "Bearer " prefix
                match auth::decode_jwt(token) {
                    Ok(claims) => {
                        let current_time = chrono::Utc::now().timestamp() as usize;
                        if current_time >= claims.exp {
                            return Ok(HttpResponse::Unauthorized().body("Token has expired"));
                        }
                        return Ok(HttpResponse::Ok().json(format!("Welcome, {}! {}", claims.sub, claims.exp)));
                    }
                    Err(_) => return Ok(HttpResponse::Unauthorized().body("Invalid token")),
                }
            }
        }
    }

    Ok(HttpResponse::Unauthorized().body("No valid token provided"))
}