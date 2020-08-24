-- Your SQL goes here
CREATE TABLE texts (
    id SERIAL PRIMARY KEY,
    site_id INT NOT NULL DEFAULT 0,
    ref_name VARCHAR NOT NULL,
    content VARCHAR NULL
    
);