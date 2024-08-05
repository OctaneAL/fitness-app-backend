use actix_web::{web, HttpResponse, Responder};
use crate::db;
use crate::models::{
    Workout, Exercise, ExerciseDetail, AddWorkout
};

async fn get_user_workouts(
    pool: web::Data<db::DbPool>,
    user_id: web::Path<String>,
) -> impl Responder {
    let client = pool.lock().await;

    let user_id: i32 = user_id.parse::<i32>().unwrap();

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

async fn add_workout(
    pool: web::Data<db::DbPool>,
    workout_id: web::Path<String>, 
    add_workout: web::Json<AddWorkout>,
) -> impl Responder {
    let mut client = pool.lock().await;
    let workout = &add_workout.workout;
    let user_id = &add_workout.user_id;
    let duration: i32 = workout.duration.parse::<i32>().unwrap();

    let transaction = client
        .transaction()
        .await
        .expect("Failed to start transaction");

    let workout_stmt = transaction
        .prepare("INSERT INTO workout (workout_id, name, date, planned_volume_kg, duration_minutes, user_id) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id")
        .await
        .expect("Error preparing workout statement");
    let workout_record_id: i32 = transaction
        .query_one(&workout_stmt, &[&workout_id.as_str(), &workout.name, &workout.date, &workout.planned_volume, &duration, &user_id])
        .await
        .expect("Error executing workout query")
        .get(0);

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

    transaction
        .commit()
        .await
        .expect("Failed to commit transaction");

    HttpResponse::Ok().json(workout)
}

async fn update_workout(
    pool: web::Data<db::DbPool>, 
    workout_id: web::Path<String>, 
    updated_workout: web::Json<Workout>
) -> impl Responder {
    let mut client = pool.lock().await;

    let transaction = client.transaction().await.expect("Error starting transaction");

    let duration: i32 = updated_workout.duration.parse::<i32>().unwrap();

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

    transaction.commit().await.expect("Error committing transaction");

    HttpResponse::Ok().json(updated_workout.into_inner())
}

async fn delete_workout(
    pool: web::Data<db::DbPool>, 
    workout_id: web::Path<String>
) -> impl Responder {
    let client = pool.lock().await;

    let delete_sets_stmt = client
        .prepare("DELETE FROM exercise_set WHERE workout_exercise_id IN (SELECT id FROM workout_exercise WHERE workout_id = (SELECT id FROM workout WHERE workout_id = $1))")
        .await
        .expect("Error preparing delete sets statement");

    client
        .execute(&delete_sets_stmt, &[&workout_id.as_str()])
        .await
        .expect("Error executing delete sets query");

    let delete_exercises_stmt = client
        .prepare("DELETE FROM workout_exercise WHERE workout_id = (SELECT id FROM workout WHERE workout_id = $1)")
        .await
        .expect("Error preparing delete exercises statement");

    client
        .execute(&delete_exercises_stmt, &[&workout_id.as_str()])
        .await
        .expect("Error executing delete exercises query");

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

async fn get_workouts(pool: web::Data<db::DbPool>) -> impl Responder {
    let client = pool.lock().await;
    
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

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg
    .service(
        web::resource("/workouts/{user_id}")
            .route(web::get().to(get_user_workouts))
    )
    .service(
        web::resource("/workout/{user_id}")
            .route(web::post().to(add_workout))
            .route(web::put().to(update_workout))
            .route(web::delete().to(delete_workout))
    )
    .service(
        web::resource("/workouts")
            .route(web::get().to(get_workouts))
    );    
}
