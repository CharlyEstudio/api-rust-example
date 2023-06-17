use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};

pub static APP_HOST: &'static str = "http://localhost:8000";

pub fn create_test_student(client: &Client) -> Value {
  let response = client
    .post(format!("{}/students", APP_HOST))
    .json(&create_json_student())
    .send()
    .unwrap();
  assert_eq!(response.status(), StatusCode::CREATED);
  response.json().unwrap()
}

pub fn delete_test_student(client: &Client, student: Value) {
  let response = client
    .delete(format!("{}/students/{}", APP_HOST, student["id"]))
    .send()
    .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

pub fn create_json_student() -> Value {
  json!({
    "name": "Foo bar",
    "email": "foo@bar.com"
  })
}

pub fn create_test_presence(client: &Client) -> Value {
  let response = client
    .post(format!("{}/assists", APP_HOST))
    .json(&create_json_presence())
    .send()
    .unwrap();
  assert_eq!(response.status(), StatusCode::CREATED);
  response.json().unwrap()
}

pub fn delete_test_presence(client: &Client, presence: Value) {
  let response = client
    .delete(format!("{}/assists/{}", APP_HOST, presence["id"]))
    .send()
    .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

pub fn create_json_presence() -> Value {
  json!({
    "students_id": 1,
    "presence": "2023-06-17T09:22:22.731"
  })
}

pub fn equal_data_presence(presence: Value) -> Value {
  json!({
    "id": presence["id"],
    "students_id": 1,
    "presence": "2023-06-17T09:22:22.731",
    "created_at": presence["created_at"],
  })
}