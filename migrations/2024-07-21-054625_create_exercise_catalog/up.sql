ALTER TABLE exercise_catalog
  ADD COLUMN short_demonstration_link VARCHAR,
  ADD COLUMN in_depth_demonstration_link VARCHAR,
  ADD COLUMN difficulty_id INTEGER,
  ADD COLUMN target_muscle_group_id INTEGER,
  ADD COLUMN prime_mover_muscle_id INTEGER,
  ADD COLUMN secondary_mover_muscle_id INTEGER,
  ADD COLUMN tertiary_mover_muscle_id INTEGER,
  ADD COLUMN primary_equipment_id INTEGER,
  ADD COLUMN primary_items_number INTEGER,
  ADD COLUMN secondary_equipment_id INTEGER,
  ADD COLUMN secondary_items_number INTEGER,
  ADD COLUMN body_region_id INTEGER;

ALTER TABLE exercise_catalog
  DROP COLUMN img_path,
  DROP COLUMN description;

-- Створення нових таблиць
CREATE TABLE muscle_group (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL
);

CREATE TABLE equipment (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL
);

CREATE TABLE body_region (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL
);

CREATE TABLE muscle (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL
);

CREATE TABLE difficulty (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL
);

CREATE TABLE "user" (
  id SERIAL PRIMARY KEY,
  username VARCHAR NOT NULL,
  password VARCHAR NOT NULL
);

ALTER TABLE exercise_catalog
ADD CONSTRAINT fk_difficulty_id
FOREIGN KEY (difficulty_id) REFERENCES difficulty(id);

ALTER TABLE exercise_catalog
ADD CONSTRAINT fk_target_muscle_group_id
FOREIGN KEY (target_muscle_group_id) REFERENCES muscle_group(id);

ALTER TABLE exercise_catalog
ADD CONSTRAINT fk_prime_mover_muscle_id
FOREIGN KEY (prime_mover_muscle_id) REFERENCES muscle(id);

ALTER TABLE exercise_catalog
ADD CONSTRAINT fk_secondary_mover_muscle_id
FOREIGN KEY (secondary_mover_muscle_id) REFERENCES muscle(id);

ALTER TABLE exercise_catalog
ADD CONSTRAINT fk_tertiary_mover_muscle_id
FOREIGN KEY (tertiary_mover_muscle_id) REFERENCES muscle(id);

ALTER TABLE exercise_catalog
ADD CONSTRAINT fk_primary_equipment_id
FOREIGN KEY (primary_equipment_id) REFERENCES equipment(id);

ALTER TABLE exercise_catalog
ADD CONSTRAINT fk_secondary_equipment_id
FOREIGN KEY (secondary_equipment_id) REFERENCES equipment(id);

ALTER TABLE exercise_catalog
ADD CONSTRAINT fk_body_region_id
FOREIGN KEY (body_region_id) REFERENCES body_region(id);
