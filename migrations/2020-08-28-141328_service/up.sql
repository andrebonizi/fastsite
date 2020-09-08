-- Your SQL goes here
CREATE TABLE service (
    id SERIAL PRIMARY KEY,
    site_id INT NOT NULL DEFAULT 0,
    content VARCHAR NULL
    
);