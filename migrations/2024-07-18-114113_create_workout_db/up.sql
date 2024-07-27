CREATE TABLE workout (
  id SERIAL PRIMARY KEY,
  workout_id VARCHAR UNIQUE,
  name VARCHAR,
  date TEXT
);

CREATE TABLE exercise_catalog (
  id SERIAL PRIMARY KEY,
  name VARCHAR,
  img_path VARCHAR,
  description VARCHAR
);

CREATE TABLE workout_exercise (
  id SERIAL PRIMARY KEY,
  workout_id INTEGER REFERENCES workout(id),
  exercise_catalog_id INTEGER REFERENCES exercise_catalog(id),
  sets_number INTEGER,
  name VARCHAR
);

CREATE TABLE exercise_set (
  id SERIAL PRIMARY KEY,
  repeats INTEGER,
  weight INTEGER,
  workout_exercise_id INTEGER REFERENCES workout_exercise(id)
);
