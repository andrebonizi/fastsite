-- Your SQL goes here
CREATE TABLE contact (
    id SERIAL PRIMARY KEY,
    site_id INT NOT NULL DEFAULT 0,
    name VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    message VARCHAR NOT NULL
);