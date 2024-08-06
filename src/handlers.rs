use actix_web::{web, HttpResponse, HttpRequest, Responder, Error, get};

use crate::db;
use crate::auth;
use crate::models::CatalogItem;
use std::collections::HashMap;

// pub fn config(cfg: &mut web::ServiceConfig, pool: &db::DbPool) {
//     cfg.service(
//         web::scope("/api")
//             .service(web::resource("/register").route(web::post().to(|user: web::Json<RegisterUser>| register_user(user))))
//             .service(web::resource("/login").route(web::post().to(|user: web::Json<LoginUser>| login_user(user))))
//             .service(web::resource("/protected").route(web::get().to(protected_endpoint))),
//     );
// }

#[get("/filter_exercises")]
async fn filter_exercises(
    pool: web::Data<db::DbPool>,
    query: web::Query<HashMap<String, String>>,
) -> impl Responder {
    let client = pool.lock().await;

    let mut query_string = "SELECT * FROM exercise_catalog WHERE 1=1".to_string();
    let mut params: Vec<Box<dyn tokio_postgres::types::ToSql + Sync>> = Vec::new();
    let mut current_index: i32 = 1;

    // Helper function to add parameter to query and params list
    fn add_param<T: tokio_postgres::types::ToSql + Sync + 'static>(
        query_string: &mut String,
        params: &mut Vec<Box<dyn tokio_postgres::types::ToSql + Sync>>,
        param_value: T,
        placeholder: &str,
        index: &mut i32,
    ) {
        let placeholder_formatted = placeholder.replace("{}", &index.to_string());
        query_string.push_str(&placeholder_formatted);
        let param = Box::new(param_value) as Box<dyn tokio_postgres::types::ToSql + Sync>;
        params.push(param);
        // *index += 1;
    }

    if let Some(target_muscle_group_id) = query.get("target_muscle_group_id") {
        add_param(
            &mut query_string,
            &mut params,
            target_muscle_group_id.parse::<i32>().unwrap(),
            " AND target_muscle_group_id = ${}",
            &mut current_index,
        );
        current_index += 1;
    }

    if let Some(name) = query.get("name") {
        let formatted_name = format!("%{}%", name);
        add_param(
            &mut query_string,
            &mut params,
            formatted_name,
            " AND name ILIKE ${}",
            &mut current_index,
        );
        current_index += 1;
    }

    if let Some(difficulty_ids) = query.get("difficulty_ids") {
        let ids: Vec<&str> = difficulty_ids.split(',').collect();
        let mut placeholders = Vec::new();
        for _ in ids.iter() {
            placeholders.push(format!("${}", current_index));
            current_index += 1;
        }
        query_string.push_str(&format!(
            " AND difficulty_id IN ({})",
            placeholders.join(", ")
        ));
        for id in ids.iter() {
            add_param(
                &mut query_string,
                &mut params,
                id.parse::<i32>().unwrap(),
                "",
                &mut current_index,
            );
        }
    }

    if let Some(equipment_ids) = query.get("equipment_ids") {
        let ids: Vec<&str> = equipment_ids.split(',').collect();
        let mut placeholders = Vec::new();
        for _ in ids.iter() {
            placeholders.push(format!("${}", current_index));
            current_index += 1;
        }
        query_string.push_str(&format!(
            " AND (primary_equipment_id IN ({}) OR secondary_equipment_id IN ({}))",
            placeholders.join(", "),
            placeholders.join(", ")
        ));
        for id in ids.iter() {
            add_param(
                &mut query_string,
                &mut params,
                id.parse::<i32>().unwrap(),
                "",
                &mut current_index,
            );
        }
    }

    if let Some(body_region_ids) = query.get("body_region_ids") {
        let ids: Vec<&str> = body_region_ids.split(',').collect();
        let mut placeholders = Vec::new();
        for _ in ids.iter() {
            placeholders.push(format!("${}", current_index));
            current_index += 1;
        }
        query_string.push_str(&format!(
            " AND body_region_id IN ({})",
            placeholders.join(", ")
        ));
        for id in ids.iter() {
            add_param(
                &mut query_string,
                &mut params,
                id.parse::<i32>().unwrap(),
                "",
                &mut current_index,
            );
        }
    }

    // Convert params to references for the query call
    let param_refs: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = params.iter().map(|p| p.as_ref()).collect();

    let rows = client
        .query(&query_string, &param_refs)
        .await
        .expect("Error executing query");

    let exercises: Vec<HashMap<String, String>> = rows.iter().map(|row| {
        let mut exercise = HashMap::new();
        exercise.insert("id".to_string(), row.get::<usize, i32>(0).to_string());
        exercise.insert("name".to_string(), row.get::<usize, String>(1));
        exercise.insert("short_demonstration_link".to_string(), row.get::<usize, Option<String>>(2).unwrap_or_default());
        exercise.insert("in_depth_demonstration_link".to_string(), row.get::<usize, Option<String>>(3).unwrap_or_default());
        exercise.insert("difficulty_id".to_string(), row.get::<usize, Option<i32>>(4).unwrap_or_default().to_string());
        // exercise.insert("target_muscle_group_id".to_string(), row.get::<usize, Option<i32>>(5).unwrap_or_default().to_string());
        // exercise.insert("prime_mover_muscle_id".to_string(), row.get::<usize, Option<i32>>(6).unwrap_or_default().to_string());
        // exercise.insert("secondary_mover_muscle_id".to_string(), row.get::<usize, Option<i32>>(7).unwrap_or_default().to_string());
        // exercise.insert("tertiary_mover_muscle_id".to_string(), row.get::<usize, Option<i32>>(8).unwrap_or_default().to_string());
        exercise.insert("primary_equipment_id".to_string(), row.get::<usize, Option<i32>>(9).unwrap_or_default().to_string());
        exercise.insert("primary_items_number".to_string(), row.get::<usize, Option<i32>>(10).unwrap_or_default().to_string());
        exercise.insert("secondary_equipment_id".to_string(), row.get::<usize, Option<i32>>(11).unwrap_or_default().to_string());
        exercise.insert("secondary_items_number".to_string(), row.get::<usize, Option<i32>>(12).unwrap_or_default().to_string());
        exercise.insert("body_region_id".to_string(), row.get::<usize, Option<i32>>(13).unwrap_or_default().to_string());
        exercise
    }).collect();

    HttpResponse::Ok().json(exercises)
}

#[get("/body_regions")]
async fn get_body_regions(
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let client = pool.lock().await;
    
    let body_regions_query = "SELECT id, name FROM body_region ORDER by name;";
    let body_regions = client
        .query(body_regions_query, &[])
        .await
        .expect("Error executing body regions query");

    let body_regions: Vec<CatalogItem> = body_regions
        .iter()
        .map(|row| CatalogItem {
            id: row.get("id"),
            name: row.get("name"),
        })
        .collect();

    HttpResponse::Ok().json(body_regions)
}

#[get("/body_regions/{muscle_group_id}")]
async fn get_body_regions_for_muscle_group(
    pool: web::Data<db::DbPool>,
    muscle_group_id: web::Path<String>,
) -> impl Responder {
    let client = pool.lock().await;

    let muscle_group_id: i32 = match muscle_group_id.parse::<i32>() {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid muscle group ID"),
    };

    let body_regions_query = "SELECT id, name FROM body_region WHERE id in (SELECT body_region_id FROM exercise_catalog WHERE target_muscle_group_id = $1 GROUP BY body_region_id) ORDER BY name;";

    match client.query(body_regions_query, &[&muscle_group_id]).await {
        Ok(rows) => {
            if rows.is_empty() {
                HttpResponse::NotFound().body("No body regions found for the given muscle group")
            } else {
                let body_regions: Vec<CatalogItem> = rows
                    .iter()
                    .map(|row| CatalogItem {
                        id: row.get("id"),
                        name: row.get("name"),
                    })
                    .collect();
                HttpResponse::Ok().json(body_regions)
            }
        }
        Err(e) => {
            eprintln!("Database query error: {:?}", e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}


#[get("/difficulties")]
async fn get_difficulties(
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let client = pool.lock().await;
    
    let difficulties_query = "SELECT id, name FROM difficulty ORDER by name;";
    let difficulties = client
        .query(difficulties_query, &[])
        .await
        .expect("Error executing difficulties query");

    let difficulties: Vec<CatalogItem> = difficulties
        .iter()
        .map(|row| CatalogItem {
            id: row.get("id"),
            name: row.get("name"),
        })
        .collect();

    HttpResponse::Ok().json(difficulties)
}

#[get("/equipment")]
async fn get_equipment(
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let client = pool.lock().await;
    
    let equipment_query = "SELECT id, name FROM equipment ORDER by name;";
    let equipment = client
        .query(equipment_query, &[])
        .await
        .expect("Error executing equipment query");

    let equipment: Vec<CatalogItem> = equipment
        .iter()
        .map(|row| CatalogItem {
            id: row.get("id"),
            name: row.get("name"),
        })
        .collect();

    HttpResponse::Ok().json(equipment)
}

#[get("/exercise_catalog")]
async fn get_exercise_catalog(pool: web::Data<db::DbPool>) -> impl Responder {
    let client = pool.lock().await;
    
    let exercise_catalog_query = "SELECT id, name FROM exercise_catalog ORDER by name;";
    let exercise_catalog = client
        .query(exercise_catalog_query, &[])
        .await
        .expect("Error executing exercise catalog query");

    let exercises: Vec<CatalogItem> = exercise_catalog
        .iter()
        .map(|row| CatalogItem {
            id: row.get("id"),
            name: row.get("name"),
        })
        .collect();

    HttpResponse::Ok().json(exercises)
}

#[get("/muscle_group_id/{muscle_group_name}")]
async fn get_muscle_group_id(
    pool: web::Data<db::DbPool>,
    muscle_group_name: web::Path<String>,
) -> impl Responder {
    let client = pool.lock().await;

    let muscle_group_name: String = muscle_group_name.parse::<String>().unwrap();

    let muscle_group_id_query = "SELECT id FROM muscle_group WHERE name = $1";

    match client.query_opt(muscle_group_id_query, &[&muscle_group_name]).await {
        Ok(Some(row)) => {
            let muscle_group_id: i32 = row.get(0);
            HttpResponse::Ok().json(muscle_group_id)
        }
        Ok(None) => {
            HttpResponse::NotFound().body("Muscle group not found")
        }
        Err(e) => {
            eprintln!("Database query error: {:?}", e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

#[get("/muscle_groups")]
async fn get_muscle_groups(pool: web::Data<db::DbPool>) -> impl Responder {
    let client = pool.lock().await;
    
    let muscle_groups_query = "SELECT name FROM muscle_group ORDER by name;";
    let muscle_groups = client
        .query(muscle_groups_query, &[])
        .await
        .expect("Error executing muscle groups query");

    let muscle_groups: Vec<String> = muscle_groups
        .iter()
        .map(|row| row.get(0))
        .collect();

    HttpResponse::Ok().json(muscle_groups)
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