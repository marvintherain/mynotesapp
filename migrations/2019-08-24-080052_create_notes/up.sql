CREATE TABLE notes (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    body TEXT,
    creation_date TIMESTAMP NOT NULL,
    last_edit TIMESTAMP NOT NULL
)