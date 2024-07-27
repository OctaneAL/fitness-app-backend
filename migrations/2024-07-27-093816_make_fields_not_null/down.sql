-- revert NOT NULL to nullable fields

ALTER TABLE exercise_catalog
    ALTER COLUMN name DROP NOT NULL;

ALTER TABLE exercise_set
    ALTER COLUMN repeats DROP NOT NULL,
    ALTER COLUMN weight DROP NOT NULL,
    ALTER COLUMN workout_exercise_id DROP NOT NULL;

ALTER TABLE workout
    ALTER COLUMN workout_id DROP NOT NULL,
    ALTER COLUMN name DROP NOT NULL,
    ALTER COLUMN date DROP NOT NULL,
    ALTER COLUMN planned_volume_kg DROP NOT NULL,
    ALTER COLUMN duration_minutes DROP NOT NULL;

ALTER TABLE workout_exercise
    ALTER COLUMN workout_id DROP NOT NULL,
    ALTER COLUMN exercise_catalog_id DROP NOT NULL;
