use actix_web::{web, HttpResponse, HttpRequest, Responder, Error, post, get, delete, put};

use crate::db;
use crate::auth;
use crate::models::{ AddWorkout, RegisterUser, LoginUser, Workout, Exercise, ExerciseDetail, ExerciseCatalogItem, LoginResponse};

// pub fn config(cfg: &mut web::ServiceConfig, pool: &db::DbPool) {
//     cfg.service(
//         web::scope("/api")
//             .service(web::resource("/register").route(web::post().to(|user: web::Json<RegisterUser>| register_user(user))))
//             .service(web::resource("/login").route(web::post().to(|user: web::Json<LoginUser>| login_user(user))))
//             .service(web::resource("/protected").route(web::get().to(protected_endpoint))),
//     );
// }

#[get("/workouts")]
async fn get_workouts(pool: web::Data<db::DbPool>) -> impl Responder {
    let client = pool.lock().await;
    
    // Запит для отримання всіх тренувань
    let workouts_query = "SELECT id, workout_id, name, date, planned_volume_kg, duration_minutes FROM workout ORDER by date;";
    let workouts = client
        .query(workouts_query, &[])
        .await
        .expect("Error executing workouts query");

    let mut workout_list = Vec::new();

    for row in workouts {
        let workout_id: i32 = row.get(0);
        let mut workout = Workout {
            id: row.get::<_, String>(1),
            name: row.get::<_, String>(2),
            date: row.get::<_, String>(3),
            planned_volume: row.get::<_, i32>(4),
            duration: row.get::<_, i32>(5).to_string(),
            exercises: Vec::new(),
        };

        // Запит для отримання вправ для кожного тренування
        let exercises_query = "SELECT id, exercise_catalog_id FROM workout_exercise WHERE workout_id = $1";
        let exercises = client
            .query(exercises_query, &[&workout_id])
            .await
            .expect("Error executing exercises query");

        for exercise_row in exercises {
            let exercise_id: i32 = exercise_row.get(0);
            let mut exercise = Exercise {
                exercise_catalog_id: exercise_row.get(1),
                details: Vec::new(),
            };

            // Запит для отримання деталей вправ для кожної вправи
            let details_query = "SELECT repeats, weight FROM exercise_set WHERE workout_exercise_id = $1";
            let details = client
                .query(details_query, &[&exercise_id])
                .await
                .expect("Error executing details query");

            for detail_row in details {
                let detail = ExerciseDetail {
                    repeats: detail_row.get::<_, i32>(0).to_string(),
                    weight: detail_row.get::<_, i32>(1).to_string(),
                };
                exercise.details.push(detail);
            }

            workout.exercises.push(exercise);
        }

        workout_list.push(workout);
    }

    HttpResponse::Ok().json(workout_list)
}

#[get("/workouts/{user_id}")]
async fn get_user_workouts(
    pool: web::Data<db::DbPool>,
    user_id: web::Path<String>,
) -> impl Responder {
    let client = pool.lock().await;

    let user_id: i32 = user_id.parse::<i32>().unwrap();

    // Запит для отримання всіх тренувань
    let workouts_query = "SELECT id, workout_id, name, date, planned_volume_kg, duration_minutes FROM workout WHERE user_id = $1 ORDER by date;";
    let workouts = client
        .query(workouts_query, &[&user_id])
        .await
        .expect("Error executing workouts query");

    let mut workout_list = Vec::new();

    for row in workouts {
        let workout_id: i32 = row.get(0);
        let mut workout = Workout {
            id: row.get::<_, String>(1),
            name: row.get::<_, String>(2),
            date: row.get::<_, String>(3),
            planned_volume: row.get::<_, i32>(4),
            duration: row.get::<_, i32>(5).to_string(),
            exercises: Vec::new(),
        };

        // Запит для отримання вправ для кожного тренування
        let exercises_query = "SELECT id, exercise_catalog_id FROM workout_exercise WHERE workout_id = $1";
        let exercises = client
            .query(exercises_query, &[&workout_id])
            .await
            .expect("Error executing exercises query");

        for exercise_row in exercises {
            let exercise_id: i32 = exercise_row.get(0);
            let mut exercise = Exercise {
                exercise_catalog_id: exercise_row.get(1),
                details: Vec::new(),
            };

            // Запит для отримання деталей вправ для кожної вправи
            let details_query = "SELECT repeats, weight FROM exercise_set WHERE workout_exercise_id = $1";
            let details = client
                .query(details_query, &[&exercise_id])
                .await
                .expect("Error executing details query");

            for detail_row in details {
                let detail = ExerciseDetail {
                    repeats: detail_row.get::<_, i32>(0).to_string(),
                    weight: detail_row.get::<_, i32>(1).to_string(),
                };
                exercise.details.push(detail);
            }

            workout.exercises.push(exercise);
        }

        workout_list.push(workout);
    }

    HttpResponse::Ok().json(workout_list)
}

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

#[post("/workout/{workout_id}")]
async fn add_workout(
    pool: web::Data<db::DbPool>,
    workout_id: web::Path<String>, 
    add_workout: web::Json<AddWorkout>,
) -> impl Responder {
    // let client = pool.get_ref();
    let mut client = pool.lock().await;
    let workout = &add_workout.workout;
    let user_id = &add_workout.user_id;
    let duration: i32 = workout.duration.parse::<i32>().unwrap();

    let transaction = client
        .transaction()
        .await
        .expect("Failed to start transaction");

    // Add workout
    let workout_stmt = transaction
        .prepare("INSERT INTO workout (workout_id, name, date, planned_volume_kg, duration_minutes, user_id) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id")
        .await
        .expect("Error preparing workout statement");
    let workout_record_id: i32 = transaction
        .query_one(&workout_stmt, &[&workout_id.as_str(), &workout.name, &workout.date, &workout.planned_volume, &duration, &user_id])
        .await
        .expect("Error executing workout query")
        .get(0);

    // Add exercises
    for exercise in &workout.exercises {
        let exercise_stmt = transaction
            .prepare("INSERT INTO workout_exercise (workout_id, exercise_catalog_id) VALUES ($1, $2) RETURNING id")
            .await
            .expect("Error preparing exercise statement");
        let exercise_id: i32 = transaction
            .query_one(&exercise_stmt, &[&workout_record_id, &exercise.exercise_catalog_id])
            .await
            .expect("Error executing exercise query")
            .get(0);

        // Add exercise details
        for detail in &exercise.details {
            let repeats: i32 = detail.repeats.parse::<i32>().unwrap();
            let weight: i32 = detail.weight.parse::<i32>().unwrap();
            let detail_stmt = transaction
                .prepare("INSERT INTO exercise_set (workout_exercise_id, repeats, weight) VALUES ($1, $2, $3)")
                .await
                .expect("Error preparing detail statement");
            transaction
                .execute(&detail_stmt, &[&exercise_id, &repeats, &weight])
                .await
                .expect("Error executing detail query");
        }
    }

    // Commit the transaction
    transaction
        .commit()
        .await
        .expect("Failed to commit transaction");

    HttpResponse::Ok().json(workout)
}

#[put("/workout/{workout_id}")]
async fn update_workout(
    pool: web::Data<db::DbPool>, 
    workout_id: web::Path<String>, 
    updated_workout: web::Json<Workout>
) -> impl Responder {
    let mut client = pool.lock().await;

    // Початок транзакції
    let transaction = client.transaction().await.expect("Error starting transaction");

    let duration: i32 = updated_workout.duration.parse::<i32>().unwrap();

    // Оновлюємо основну інформацію про тренування
    let update_workout_stmt = transaction
        .prepare("UPDATE workout SET name = $1, date = $2, planned_volume_kg = $3, duration_minutes = $4 WHERE workout_id = $5 RETURNING id")
        .await
        .expect("Error preparing update workout statement");

    let workout_record_id: i32 = transaction
        .query_one(
            &update_workout_stmt, 
            &[&updated_workout.name, &updated_workout.date, &updated_workout.planned_volume, &duration, &workout_id.as_str()]
        )
        .await
        .expect("Error executing update workout query")
        .get(0);

    // Видаляємо старі дані про вправи та підходи
    let delete_sets_stmt = transaction
        .prepare("DELETE FROM exercise_set WHERE workout_exercise_id IN (SELECT id FROM workout_exercise WHERE workout_id = (SELECT id FROM workout WHERE workout_id = $1))")
        .await
        .expect("Error preparing delete sets statement");

    transaction
        .execute(&delete_sets_stmt, &[&workout_id.as_str()])
        .await
        .expect("Error executing delete sets query");

    let delete_exercises_stmt = transaction
        .prepare("DELETE FROM workout_exercise WHERE workout_id = (SELECT id FROM workout WHERE workout_id = $1)")
        .await
        .expect("Error preparing delete exercises statement");

    transaction
        .execute(&delete_exercises_stmt, &[&workout_id.as_str()])
        .await
        .expect("Error executing delete exercises query");

    // Додаємо оновлені дані про вправи та підходи
    for exercise in &updated_workout.exercises {
        let exercise_stmt = transaction
            .prepare("INSERT INTO workout_exercise (workout_id, exercise_catalog_id) VALUES ($1, $2) RETURNING id")
            .await
            .expect("Error preparing insert exercise statement");

        let exercise_id: i32 = transaction
            .query_one(&exercise_stmt, &[&workout_record_id, &exercise.exercise_catalog_id])
            .await
            .expect("Error executing insert exercise query")
            .get(0);

        for detail in &exercise.details {
            let repeats: i32 = detail.repeats.parse::<i32>().unwrap();
            let weight: i32 = detail.weight.parse::<i32>().unwrap();
            let detail_stmt = transaction
                .prepare("INSERT INTO exercise_set (workout_exercise_id, repeats, weight) VALUES ($1, $2, $3)")
                .await
                .expect("Error preparing insert detail statement");

            transaction
                .execute(&detail_stmt, &[&exercise_id, &repeats, &weight])
                .await
                .expect("Error executing insert detail query");
        }
    }

    // Підтверджуємо транзакцію
    transaction.commit().await.expect("Error committing transaction");

    HttpResponse::Ok().json(updated_workout.into_inner())
}

#[delete("/workout/{workout_id}")]
async fn delete_workout(
    pool: web::Data<db::DbPool>, 
    workout_id: web::Path<String>
) -> impl Responder {
    // let client = pool.get_ref();
    let client = pool.lock().await;

    // Видаляємо підходи, пов'язані з вправами тренування
    let delete_sets_stmt = client
        .prepare("DELETE FROM exercise_set WHERE workout_exercise_id IN (SELECT id FROM workout_exercise WHERE workout_id = (SELECT id FROM workout WHERE workout_id = $1))")
        .await
        .expect("Error preparing delete sets statement");

    client
        .execute(&delete_sets_stmt, &[&workout_id.as_str()])
        .await
        .expect("Error executing delete sets query");

    // Видаляємо вправи, пов'язані з тренуванням
    let delete_exercises_stmt = client
        .prepare("DELETE FROM workout_exercise WHERE workout_id = (SELECT id FROM workout WHERE workout_id = $1)")
        .await
        .expect("Error preparing delete exercises statement");

    client
        .execute(&delete_exercises_stmt, &[&workout_id.as_str()])
        .await
        .expect("Error executing delete exercises query");

    // Видаляємо саме тренування
    let delete_workout_stmt = client
        .prepare("DELETE FROM workout WHERE workout_id = $1")
        .await
        .expect("Error preparing delete workout statement");

    client
        .execute(&delete_workout_stmt, &[&workout_id.as_str()])
        .await
        .expect("Error executing delete workout query");

    HttpResponse::Ok().json("Workout deleted successfully")
}

#[post("/register")]
async fn register_user(pool: web::Data<db::DbPool>, user: web::Json<RegisterUser>) -> impl Responder {
    // let client = pool.get_ref();
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

#[post("/login")]
async fn login_user(pool: web::Data<db::DbPool>, user: web::Json<LoginUser>) -> impl Responder {
    // let client = pool.get_ref();
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