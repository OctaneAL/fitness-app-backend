// @generated automatically by Diesel CLI.

diesel::table! {
    body_region (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    difficulty (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    equipment (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    exercise_catalog (id) {
        id -> Int4,
        name -> Varchar,
        short_demonstration_link -> Nullable<Varchar>,
        in_depth_demonstration_link -> Nullable<Varchar>,
        difficulty_id -> Nullable<Int4>,
        target_muscle_group_id -> Nullable<Int4>,
        prime_mover_muscle_id -> Nullable<Int4>,
        secondary_mover_muscle_id -> Nullable<Int4>,
        tertiary_mover_muscle_id -> Nullable<Int4>,
        primary_equipment_id -> Nullable<Int4>,
        primary_items_number -> Nullable<Int4>,
        secondary_equipment_id -> Nullable<Int4>,
        secondary_items_number -> Nullable<Int4>,
        body_region_id -> Nullable<Int4>,
    }
}

diesel::table! {
    exercise_set (id) {
        id -> Int4,
        repeats -> Int4,
        weight -> Int4,
        workout_exercise_id -> Int4,
    }
}

diesel::table! {
    favorite_exercise (id) {
        id -> Int4,
        user_id -> Int4,
        exercise_catalog_id -> Int4,
    }
}

diesel::table! {
    muscle (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    muscle_group (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    user (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
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
        workout_id -> Varchar,
        name -> Varchar,
        date -> Text,
        user_id -> Nullable<Int4>,
        planned_volume_kg -> Int4,
        duration_minutes -> Int4,
    }
}

diesel::table! {
    workout_exercise (id) {
        id -> Int4,
        workout_id -> Int4,
        exercise_catalog_id -> Int4,
    }
}

diesel::joinable!(exercise_catalog -> body_region (body_region_id));
diesel::joinable!(exercise_catalog -> difficulty (difficulty_id));
diesel::joinable!(exercise_catalog -> muscle_group (target_muscle_group_id));
diesel::joinable!(exercise_set -> workout_exercise (workout_exercise_id));
diesel::joinable!(favorite_exercise -> exercise_catalog (exercise_catalog_id));
diesel::joinable!(favorite_exercise -> users (user_id));
diesel::joinable!(workout -> users (user_id));
diesel::joinable!(workout_exercise -> exercise_catalog (exercise_catalog_id));
diesel::joinable!(workout_exercise -> workout (workout_id));

diesel::allow_tables_to_appear_in_same_query!(
    body_region,
    difficulty,
    equipment,
    exercise_catalog,
    exercise_set,
    favorite_exercise,
    muscle,
    muscle_group,
    user,
    users,
    workout,
    workout_exercise,
);
