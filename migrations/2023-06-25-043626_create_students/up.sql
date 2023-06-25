CREATE TABLE IF NOT EXISTS students (
  id SERIAL PRIMARY KEY,
  person_id integer NOT NULL REFERENCES people(id),
  category_id integer NOT NULL REFERENCES categories(id),
  created_at TIMESTAMP DEFAULT NOW() NOT NULL
)
