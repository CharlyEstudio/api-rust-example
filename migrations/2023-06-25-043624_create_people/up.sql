CREATE TABLE IF NOT EXISTS people (
  id SERIAL PRIMARY KEY,
  name VARCHAR(64) NOT NULL UNIQUE,
  first_name VARCHAR(150) NOT NULL,
  surname VARCHAR(150),
  user_id integer NOT NULL REFERENCES users(id),
  parent_id integer NOT NULL REFERENCES users(id),
  created_at TIMESTAMP DEFAULT NOW() NOT NULL
)
