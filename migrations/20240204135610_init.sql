-- Add migration script here
CREATE TABLE IF NOT EXISTS todos (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    start_time TIMESTAMP WITH TIME ZONE NOT NULL
);
