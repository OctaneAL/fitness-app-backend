// @generated automatically by Diesel CLI.

diesel::table! {
    exercise_catalog (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        img_path -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
    }
}

diesel::table! {
    exercise_set (id) {
        id -> Int4,
        repeats -> Nullable<Int4>,
        weight -> Nullable<Int4>,
        workout_exercise_id -> Nullable<Int4>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
    }
}

diesel::table! {
    workout (id) {
        id -> Int4,
        workout_id -> Nullable<Varchar>,
        name -> Nullable<Varchar>,
        date -> Nullable<Date>,
    }
}

diesel::table! {
    workout_exercise (id) {
        id -> Int4,
        workout_id -> Nullable<Int4>,
        exercise_catalog_id -> Nullable<Int4>,
        sets_number -> Nullable<Int4>,
        name -> Nullable<Varchar>,
    }
}

diesel::joinable!(exercise_set -> workout_exercise (workout_exercise_id));
diesel::joinable!(workout_exercise -> exercise_catalog (exercise_catalog_id));
diesel::joinable!(workout_exercise -> workout (workout_id));

diesel::allow_tables_to_appear_in_same_query!(
    exercise_catalog,
    exercise_set,
    users,
    workout,
    workout_exercise,
);
