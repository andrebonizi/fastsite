-- Your SQL goes here
CREATE TABLE images (
    id SERIAL PRIMARY KEY,
    site_id INT NOT NULL DEFAULT 0,
    ref_name VARCHAR NOT NULL,
    ref_path VARCHAR NULL
);