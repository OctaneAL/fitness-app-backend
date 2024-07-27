ALTER TABLE workout_exercise
    ADD COLUMN sets_number,
    ADD COLUMN name;

ALTER TABLE workout
    DROP COLUMN user_id INTEGER,
    DROP COLUMN planned_volume_kg INTEGER,
    DROP COLUMN duration_minutes INTEGER,
    DROP CONSTRAINT workout_user_id_fkey;