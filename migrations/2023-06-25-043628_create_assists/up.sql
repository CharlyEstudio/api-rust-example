CREATE TABLE IF NOT EXISTS assists (
  id SERIAL PRIMARY KEY,
  students_id integer NOT NULL REFERENCES students(id),
  presence BOOLEAN,
  created_at TIMESTAMP DEFAULT NOW() NOT NULL
)
