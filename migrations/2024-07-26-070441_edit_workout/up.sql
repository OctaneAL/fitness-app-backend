ALTER TABLE workout_exercise
    DROP COLUMN sets_number,
    DROP COLUMN name;

ALTER TABLE workout
    ADD COLUMN user_id INTEGER,
    ADD COLUMN planned_volume_kg INTEGER,
    ADD COLUMN duration_minutes INTEGER,
    ADD CONSTRAINT workout_user_id_fkey FOREIGN KEY (user_id) REFERENCES "user"(id);