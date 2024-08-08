use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use std::collections::HashMap;

use crate::schema::users;

pub type IdToName = HashMap<i32, String>;
pub type StringCounter = HashMap<String, i32>;

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Distribution {
    pub name: String,
    pub value: i32,
}

#[derive(Serialize, Deserialize)]
pub struct FavoriteRequest {
    pub exercise_catalog_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct CatalogItem {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct ExerciseDetail {
    pub repeats: String,
    pub weight: String,
}

#[derive(Serialize, Deserialize)]
pub struct Exercise {
    pub exercise_catalog_id: i32,
    pub details: Vec<ExerciseDetail>,
}

#[derive(Serialize, Deserialize)]
pub struct Workout {
    pub id: String,
    pub name: String,
    pub date: String,
    pub planned_volume: i32,
    pub duration: String,
    pub exercises: Vec<Exercise>,
}

#[derive(Serialize, Deserialize)]
pub struct AddWorkout {
    pub workout: Workout,
    pub user_id: i32,
}

// #[derive(Queryable, Serialize, Deserialize)]
// pub struct WorkoutPlan {
//     pub id: i32,
//     pub name: String,
//     pub exercises: Vec<String>,
//     pub user_id: i32,
// }

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

// #[derive(Insertable, Deserialize)]
// #[table_name = "workout_plans"]
// pub struct NewWorkoutPlan<'a> {
//     pub name: &'a str,
//     pub exercises: &'a Vec<String>,
//     pub user_id: i32,
// }

#[derive(Serialize, Deserialize)]
pub struct RegisterUser {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}