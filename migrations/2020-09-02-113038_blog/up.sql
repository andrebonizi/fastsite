-- Your SQL goes here
CREATE TABLE blog (
    id SERIAL PRIMARY KEY,
    site_id INT NOT NULL DEFAULT 0,
    post VARCHAR NULL,
    img_path VARCHAR NULL,
    posted_at TIMESTAMP NOT NULL
    
);