use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};

pub mod common;

#[test]
fn test_get_students() {
  // Setup
  let client = Client::new();
  let student1 = common::create_test_student(&client);
  let student2 = common::create_test_student(&client);

  // Test
  let response = client
    .get(format!("{}:8000/students", common::APP_HOST))
    .send()
    .unwrap();
  assert_eq!(response.status(), StatusCode::OK);

  let json: Value = response.json().unwrap();
  assert!(json.as_array().unwrap().contains(&student1));
  assert!(json.as_array().unwrap().contains(&student2));

  // Cleanup
  common::delete_test_student(&client, student1);
  common::delete_test_student(&client, student2);
}

#[test]
fn test_create_student() {
  // Setup
  let client = Client::new();
  let response = client
    .post(format!("{}/students", common::APP_HOST))
    .json(&common::create_json_student())
    .send()
    .unwrap();
  assert_eq!(response.status(), StatusCode::CREATED);

  // Test
  let student: Value = response.json().unwrap();
  assert_eq!(student, json!({
    "id": student["id"],
    "name": "Foo bar",
    "email": "foo@bar.com",
    "created_at": student["created_at"],
  }));

  // Cleanup
  common::delete_test_student(&client, student);
}

#[test]
fn test_view_student() {
  // Setup
  let client = Client::new();
  let student: Value = common::create_test_student(&client);

  // Test
  let response = client
    .get(format!("{}/students/{}", common::APP_HOST, student["id"]))
    .send()
    .unwrap();

  assert_eq!(response.status(), StatusCode::OK);
  let student: Value = response.json().unwrap();
  assert_eq!(student, json!({
    "id": student["id"],
    "name": "Foo bar",
    "email": "foo@bar.com",
    "created_at": student["created_at"],
  }));

  // Cleanup
  common::delete_test_student(&client, student);
}

#[test]
fn test_update_student() {
  // Setup
  let client = Client::new();
  let student: Value = common::create_test_student(&client);

  // Test
  let response = client
    .put(format!("{}/students/{}", common::APP_HOST, student["id"]))
    .json(&common::create_json_student())
    .send()
    .unwrap();

  assert_eq!(response.status(), StatusCode::OK);
  let student: Value = response.json().unwrap();
  assert_eq!(student, json!({
    "id": student["id"],
    "name": "Fooz bar",
    "email": "fooz@bar.com",
    "created_at": student["created_at"],
  }));
  
  // Cleanup
  common::delete_test_student(&client, student);
}

#[test]
fn test_delete_student() {
  let client = Client::new();
  let student: Value = common::create_test_student(&client);

  let response = client
    .delete(format!("{}/students/{}", common::APP_HOST, student["id"]))
    .send()
    .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}