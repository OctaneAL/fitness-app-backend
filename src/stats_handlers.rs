use actix_web::{web, HttpResponse, Responder};
use crate::db;
use crate::models::{
    Distribution, IdToName, StringCounter
};

use std::collections::HashMap;

async fn get_user_total_workouts(
    pool: web::Data<db::DbPool>,
    user_id: web::Path<String>,
) -> impl Responder {
    let client = pool.lock().await;

    let user_id: i32 = user_id.parse::<i32>().unwrap();

    let total_workouts_query = "SELECT COUNT(*) FROM workout WHERE user_id = $1;";
    let total_workouts: i64 = client
        .query_one(total_workouts_query, &[&user_id])
        .await
        .expect("Error executing workouts query")
        .get(0);

    HttpResponse::Ok().json(total_workouts)
}

async fn get_user_total_duration(
    pool: web::Data<db::DbPool>,
    user_id: web::Path<String>,
) -> impl Responder {
    let client = pool.lock().await;

    let user_id: i32 = user_id.parse::<i32>().unwrap();

    let total_duration_query = "SELECT SUM(duration_minutes) FROM workout WHERE user_id = $1;";
    let total_duration: i64 = client
        .query_one(total_duration_query, &[&user_id])
        .await
        .expect("Error executing workouts query")
        .get(0);

    HttpResponse::Ok().json(total_duration)
}

async fn get_user_total_sets(
    pool: web::Data<db::DbPool>,
    user_id: web::Path<String>,
) -> impl Responder {
    let client = pool.lock().await;

    let user_id: i32 = user_id.parse::<i32>().unwrap();

    let workouts_query = "SELECT id FROM workout WHERE user_id = $1;";
    let workouts = client
        .query(workouts_query, &[&user_id])
        .await
        .expect("Error executing workouts query");
    
    let mut total_sets: i64 = 0;
    for row in workouts {
        let workout_id: i32 = row.get(0);

        let exercises_query = "SELECT id FROM workout_exercise WHERE workout_id = $1";
        let exercises = client
            .query(exercises_query, &[&workout_id])
            .await
            .expect("Error executing exercises query");

        for exercise_row in exercises {
            let exercise_id: i32 = exercise_row.get(0);

            let sets_query = "SELECT COUNT(*) FROM exercise_set WHERE workout_exercise_id = $1";

            let sets: i64 = client
                .query_one(sets_query, &[&exercise_id])
                .await
                .expect("Error executing workouts query")
                .get(0);

            total_sets += sets;
        }
    }

    HttpResponse::Ok().json(total_sets)
}

async fn get_user_total_weight(
    pool: web::Data<db::DbPool>,
    user_id: web::Path<String>,
) -> impl Responder {
    let client = pool.lock().await;

    let user_id: i32 = user_id.parse::<i32>().unwrap();

    let total_weight_query = "SELECT SUM(planned_volume_kg) FROM workout WHERE user_id = $1;";
    let total_weight: i64 = client
        .query_one(total_weight_query, &[&user_id])
        .await
        .expect("Error executing workouts query")
        .get(0);

    HttpResponse::Ok().json(total_weight)
}

async fn get_user_difficulty_distribution(
    pool: web::Data<db::DbPool>,
    user_id: web::Path<String>,
) -> impl Responder {
    let client = pool.lock().await;

    let user_id: i32 = user_id.parse::<i32>().unwrap();

    let mut counter: StringCounter = HashMap::new();
    let mut difficulty_id_to_name: IdToName = HashMap::new();

    let difficulty_data_query = "SELECT id, name FROM difficulty;";
    let difficulty_data = client
        .query(difficulty_data_query, &[])
        .await
        .expect("Error executing difficulty data query");

    for difficulty_row in difficulty_data {
        let difficulty_id: i32 = difficulty_row.get(0);
        let difficulty_name: String = difficulty_row.get(1);

        difficulty_id_to_name.insert(difficulty_id, difficulty_name);
    }

    let workouts_query = "SELECT id FROM workout WHERE user_id = $1;";
    let workouts = client
        .query(workouts_query, &[&user_id])
        .await
        .expect("Error executing workouts query");

    for workout_row in workouts {
        let workout_id: i32 = workout_row.get(0);

        let exercises_query = "SELECT exercise_catalog_id FROM workout_exercise WHERE workout_id = $1;";
        let exercises = client
            .query(exercises_query, &[&workout_id])
            .await
            .expect("Error executing exercises query");

        for exercise_row in exercises {
            let exercise_id: i32 = exercise_row.get(0);

            let difficulty_id_query = "SELECT difficulty_id FROM exercise_catalog WHERE id = $1;";
            let difficulty_id: i32 = client
                .query_one(difficulty_id_query, &[&exercise_id])
                .await
                .expect("Error executing difficulty_id query")
                .get(0);

            let difficulty_name: String = difficulty_id_to_name.get(&difficulty_id).unwrap().clone();

            let count = counter.entry(difficulty_name).or_insert(0);
            *count += 1;
        }
    }

    let difficulty_distribution_list: Vec<Distribution> = counter.into_iter()
        .map(|(name, value)| Distribution { name, value })
        .collect();

    HttpResponse::Ok().json(difficulty_distribution_list)
}

async fn get_user_muscle_group_exercise_distribution(
    pool: web::Data<db::DbPool>,
    user_id: web::Path<String>,
) -> impl Responder {
    let client = pool.lock().await;

    let user_id: i32 = user_id.parse::<i32>().unwrap();

    let mut counter: StringCounter = HashMap::new();
    let mut muscle_group_id_to_name: IdToName = HashMap::new();

    let muscle_group_data_query = "SELECT id, name FROM muscle_group;";
    let muscle_group_data = client
        .query(muscle_group_data_query, &[])
        .await
        .expect("Error executing difficulty data query");

    for muscle_group_row in muscle_group_data {
        let muscle_group_id: i32 = muscle_group_row.get(0);
        let muscle_group_name: String = muscle_group_row.get(1);

        muscle_group_id_to_name.insert(muscle_group_id, muscle_group_name);
    }

    let workouts_query = "SELECT id FROM workout WHERE user_id = $1;";
    let workouts = client
        .query(workouts_query, &[&user_id])
        .await
        .expect("Error executing workouts query");

    for workout_row in workouts {
        let workout_id: i32 = workout_row.get(0);

        let exercises_query = "SELECT exercise_catalog_id FROM workout_exercise WHERE workout_id = $1;";
        let exercises = client
            .query(exercises_query, &[&workout_id])
            .await
            .expect("Error executing exercises query");

        for exercise_row in exercises {
            let exercise_id: i32 = exercise_row.get(0);

            let muscle_group_id_query = "SELECT target_muscle_group_id FROM exercise_catalog WHERE id = $1;";
            let muscle_group_id: i32 = client
                .query_one(muscle_group_id_query, &[&exercise_id])
                .await
                .expect("Error executing muscle_group_id query")
                .get(0);

            let muscle_group_name: String = muscle_group_id_to_name.get(&muscle_group_id).unwrap().clone();

            let count = counter.entry(muscle_group_name).or_insert(0);
            *count += 1;
        }
    }

    let muscle_group_exercise_distribution_list: Vec<Distribution> = counter.into_iter()
        .map(|(name, value)| Distribution { name, value })
        .collect();

    HttpResponse::Ok().json(muscle_group_exercise_distribution_list)
}

async fn get_user_muscle_group_weight_distribution(
    pool: web::Data<db::DbPool>,
    user_id: web::Path<String>,
) -> impl Responder {
    let client = pool.lock().await;

    let user_id: i32 = user_id.parse::<i32>().unwrap();

    let mut counter: StringCounter = HashMap::new();
    let mut muscle_group_id_to_name: IdToName = HashMap::new();

    let muscle_group_data_query = "SELECT id, name FROM muscle_group;";
    let muscle_group_data = client
        .query(muscle_group_data_query, &[])
        .await
        .expect("Error executing difficulty data query");

    for muscle_group_row in muscle_group_data {
        let muscle_group_id: i32 = muscle_group_row.get(0);
        let muscle_group_name: String = muscle_group_row.get(1);

        muscle_group_id_to_name.insert(muscle_group_id, muscle_group_name);
    }

    let workouts_query = "SELECT id FROM workout WHERE user_id = $1;";
    let workouts = client
        .query(workouts_query, &[&user_id])
        .await
        .expect("Error executing workouts query");

    for workout_row in workouts {
        let workout_id: i32 = workout_row.get(0);

        let exercises_query = "SELECT id, exercise_catalog_id FROM workout_exercise WHERE workout_id = $1;";
        let exercises = client
            .query(exercises_query, &[&workout_id])
            .await
            .expect("Error executing exercises query");

        for exercise_row in exercises {
            let workout_exercise_id: i32 = exercise_row.get(0);
            let exercise_catalog_id: i32 = exercise_row.get(1);

            let muscle_group_id_query = "SELECT target_muscle_group_id FROM exercise_catalog WHERE id = $1;";
            let muscle_group_id: i32 = client
                .query_one(muscle_group_id_query, &[&exercise_catalog_id])
                .await
                .expect("Error executing muscle_group_id query")
                .get(0);

            let muscle_group_name: String = muscle_group_id_to_name.get(&muscle_group_id).unwrap().clone();

            let sets_query = "SELECT weight, repeats FROM exercise_set WHERE workout_exercise_id = $1;";
            let sets = client
                .query(sets_query, &[&workout_exercise_id])
                .await
                .expect("Error executing sets query");
            
            let mut lifted_weight: i32 = 0;
            for set_row in sets {
                let weight: i32 = set_row.get(0);
                let repeats: i32 = set_row.get(1);

                lifted_weight += weight * repeats;
            }

            let count = counter.entry(muscle_group_name).or_insert(0);
            *count += lifted_weight;
        }
    }

    let muscle_group_exercise_distribution_list: Vec<Distribution> = counter.into_iter()
        .map(|(name, value)| Distribution { name, value })
        .collect();

    HttpResponse::Ok().json(muscle_group_exercise_distribution_list)
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg
    .service(
        web::resource("/statistics/total-workouts/{user_id}")
            .route(web::get().to(get_user_total_workouts))
    )
    .service(
        web::resource("/statistics/total-duration/{user_id}")
            .route(web::get().to(get_user_total_duration))
    )
    .service(
        web::resource("/statistics/total-sets/{user_id}")
            .route(web::get().to(get_user_total_sets))
    )
    .service(
        web::resource("/statistics/total-weight/{user_id}")
            .route(web::get().to(get_user_total_weight))
    )
    .service(
        web::resource("/statistics/difficulty-distribution/{user_id}")
            .route(web::get().to(get_user_difficulty_distribution))
    )
    .service(
        web::resource("/statistics/muscle-group-exercise-distribution/{user_id}")
            .route(web::get().to(get_user_muscle_group_exercise_distribution))
    )
    .service(
        web::resource("/statistics/muscle-group-weight-distribution/{user_id}")
            .route(web::get().to(get_user_muscle_group_weight_distribution))
    );    
}
