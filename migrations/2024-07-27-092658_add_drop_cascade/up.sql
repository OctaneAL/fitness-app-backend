ALTER TABLE workout
    DROP CONSTRAINT workout_user_id_fkey,
    ADD CONSTRAINT workout_user_id_fkey FOREIGN KEY (user_id) REFERENCES "users"(id);

ALTER TABLE exercise_set
    DROP CONSTRAINT IF EXISTS exercise_set_workout_exercise_id_fkey,
    ADD CONSTRAINT exercise_set_workout_exercise_id_fkey
    FOREIGN KEY (workout_exercise_id)
    REFERENCES workout_exercise (id)
    ON DELETE CASCADE;

ALTER TABLE workout
    DROP CONSTRAINT IF EXISTS workout_user_id_fkey,
    ADD CONSTRAINT workout_user_id_fkey
    FOREIGN KEY (user_id)
    REFERENCES users (id)
    ON DELETE CASCADE;

ALTER TABLE workout_exercise
    DROP CONSTRAINT IF EXISTS workout_exercise_exercise_catalog_id_fkey,
    ADD CONSTRAINT workout_exercise_exercise_catalog_id_fkey
    FOREIGN KEY (exercise_catalog_id)
    REFERENCES exercise_catalog (id)
    ON DELETE CASCADE;

ALTER TABLE workout_exercise
    DROP CONSTRAINT IF EXISTS workout_exercise_workout_id_fkey,
    ADD CONSTRAINT workout_exercise_workout_id_fkey
    FOREIGN KEY (workout_id)
    REFERENCES workout (id)
    ON DELETE CASCADE;