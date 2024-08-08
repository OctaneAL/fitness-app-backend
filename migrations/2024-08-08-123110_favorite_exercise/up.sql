CREATE TABLE favorite_exercise (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    exercise_catalog_id INTEGER NOT NULL REFERENCES exercise_catalog(id)
);
