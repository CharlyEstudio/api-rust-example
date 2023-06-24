pub struct ServerErrorProps {
  pub service: String,
  pub id: i32,
  pub table: String,
}

impl ServerErrorProps {
  pub fn new(service: String, id: i32, table: String) -> Self {
    Self { service, id, table }
  }
}