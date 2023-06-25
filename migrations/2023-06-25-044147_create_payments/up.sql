CREATE TABLE IF NOT EXISTS payments (
  id SERIAL PRIMARY KEY,
  student_id integer NOT NULL REFERENCES students(id),
  amount REAL NOT NULL DEFAULT 0.0,
  type_payment_id integer NOT NULL REFERENCES type_payments(id),
  service_id integer,
  created_at TIMESTAMP DEFAULT NOW() NOT NULL
)
