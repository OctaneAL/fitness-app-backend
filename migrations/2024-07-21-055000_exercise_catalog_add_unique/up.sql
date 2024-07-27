ALTER TABLE exercise_catalog ADD CONSTRAINT unique_exercise_name UNIQUE (name);
ALTER TABLE muscle_group ADD CONSTRAINT unique_muscle_group_name UNIQUE (name);
ALTER TABLE equipment ADD CONSTRAINT unique_equipment_name UNIQUE (name);
ALTER TABLE body_region ADD CONSTRAINT unique_body_region_name UNIQUE (name);
ALTER TABLE muscle ADD CONSTRAINT unique_muscle_name UNIQUE (name);
ALTER TABLE difficulty ADD CONSTRAINT unique_difficulty_name UNIQUE (name);
ALTER TABLE "user" ADD CONSTRAINT unique_username UNIQUE (username);
