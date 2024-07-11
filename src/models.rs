use serde::{Deserialize, Serialize};
use diesel::prelude::*;

use crate::schema::users;

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
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