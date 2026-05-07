CREATE TABLE IF NOT EXISTS users (
     id SERIAL PRIMARY KEY,
     username TEXT NOT NULL UNIQUE,
     password TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS todos (
    todo_id TEXT PRIMARY KEY,
    username TEXT NOT NULL,
    description TEXT NOT NULL,
    status BOOLEAN NOT NULL DEFAULT false
);