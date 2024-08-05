use actix_web::{web, HttpResponse, HttpRequest, Responder, Error, get};

use crate::db;
use crate::auth;
use crate::models::ExerciseCatalogItem;

// pub fn config(cfg: &mut web::ServiceConfig, pool: &db::DbPool) {
//     cfg.service(
//         web::scope("/api")
//             .service(web::resource("/register").route(web::post().to(|user: web::Json<RegisterUser>| register_user(user))))
//             .service(web::resource("/login").route(web::post().to(|user: web::Json<LoginUser>| login_user(user))))
//             .service(web::resource("/protected").route(web::get().to(protected_endpoint))),
//     );
// }

#[get("/exercise_catalog")]
async fn get_exercise_catalog(pool: web::Data<db::DbPool>) -> impl Responder {
    let client = pool.lock().await;
    
    let exercise_catalog_query = "SELECT id, name FROM exercise_catalog ORDER by name;";
    let exercise_catalog = client
        .query(exercise_catalog_query, &[])
        .await
        .expect("Error executing workouts query");

    let exercises: Vec<ExerciseCatalogItem> = exercise_catalog
        .iter()
        .map(|row| ExerciseCatalogItem {
            id: row.get("id"),
            name: row.get("name"),
        })
        .collect();

    HttpResponse::Ok().json(exercises)
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