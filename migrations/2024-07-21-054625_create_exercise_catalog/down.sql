ALTER TABLE exercise_catalog DROP CONSTRAINT fk_difficulty_id;
ALTER TABLE exercise_catalog DROP CONSTRAINT fk_target_muscle_group_id;
ALTER TABLE exercise_catalog DROP CONSTRAINT fk_prime_mover_muscle_id;
ALTER TABLE exercise_catalog DROP CONSTRAINT fk_secondary_mover_muscle_id;
ALTER TABLE exercise_catalog DROP CONSTRAINT fk_tertiary_mover_muscle_id;
ALTER TABLE exercise_catalog DROP CONSTRAINT fk_primary_equipment_id;
ALTER TABLE exercise_catalog DROP CONSTRAINT fk_secondary_equipment_id;
ALTER TABLE exercise_catalog DROP CONSTRAINT fk_body_region_id;

ALTER TABLE exercise_catalog
  DROP COLUMN short_demonstration_link,
  DROP COLUMN in_depth_demonstration_link,
  DROP COLUMN difficulty_id,
  DROP COLUMN target_muscle_group_id,
  DROP COLUMN prime_mover_muscle_id,
  DROP COLUMN secondary_mover_muscle_id,
  DROP COLUMN tertiary_mover_muscle_id,
  DROP COLUMN primary_equipment_id,
  DROP COLUMN primary_items_number,
  DROP COLUMN secondary_equipment_id,
  DROP COLUMN secondary_items_number,
  DROP COLUMN body_region_id;

ALTER TABLE exercise_catalog
  ADD COLUMN img_path VARCHAR,
  ADD COLUMN description VARCHAR;

DROP TABLE IF EXISTS muscle_group;
DROP TABLE IF EXISTS equipment;
DROP TABLE IF EXISTS body_region;
DROP TABLE IF EXISTS muscle;
DROP TABLE IF EXISTS difficulty;
DROP TABLE IF EXISTS "user";
