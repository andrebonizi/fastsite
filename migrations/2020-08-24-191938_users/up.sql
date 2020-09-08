-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    site_id INT NOT NULL DEFAULT 0,
    ref_name VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    pass VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL,
    lastlogged_at TIMESTAMP NOT NULL
    
);