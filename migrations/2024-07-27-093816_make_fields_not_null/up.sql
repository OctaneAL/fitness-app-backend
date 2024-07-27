-- make fields NOT NULL

ALTER TABLE exercise_catalog
    ALTER COLUMN name SET NOT NULL;

ALTER TABLE exercise_set
    ALTER COLUMN repeats SET NOT NULL,
    ALTER COLUMN weight SET NOT NULL,
    ALTER COLUMN workout_exercise_id SET NOT NULL;

ALTER TABLE workout
    ALTER COLUMN workout_id SET NOT NULL,
    ALTER COLUMN name SET NOT NULL,
    ALTER COLUMN date SET NOT NULL,
    ALTER COLUMN planned_volume_kg SET NOT NULL,
    ALTER COLUMN duration_minutes SET NOT NULL;

ALTER TABLE workout_exercise
    ALTER COLUMN workout_id SET NOT NULL,
    ALTER COLUMN exercise_catalog_id SET NOT NULL;