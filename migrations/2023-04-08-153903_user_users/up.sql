-- Your SQL goes here

CREATE TABLE user_users (
  id SERIAL PRIMARY KEY,
  key VARCHAR(255) UNIQUE NOT NULL,
  role VARCHAR(10) NOT NULL CHECK (role IN ('Admin', 'User'))
);