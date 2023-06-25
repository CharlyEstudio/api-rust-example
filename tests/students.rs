use reqwest::{StatusCode, blocking::Client};
use serde_json::{json, Value};

pub mod common;

#[test]
fn test_get_students() {
  // Setup
  let client = common::get_client_with_logged_in_admin();
  let student1 = common::create_test_student(&client);
  let student2 = common::create_test_student(&client);

  // Test
  let response = client
    .get(format!("{}/students", common::APP_HOST))
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
fn test_get_students_not_authorized() {
  // Setup
  let client = Client::new();

  // Test
  let response = client
    .get(format!("{}/students", common::APP_HOST))
    .send()
    .unwrap();
  assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[test]
fn test_create_student() {
  // Setup
  let client = common::get_client_with_logged_in_admin();
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
  let client = common::get_client_with_logged_in_admin();
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
fn test_view_student_not_found() {
  // Setup
  let client = common::get_client_with_logged_in_admin();

  // Test
  let response = client
    .get(format!("{}/students/{}", common::APP_HOST, 100000000))
    .send()
    .unwrap();

  assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[test]
fn test_update_student() {
  // Setup
  let client = common::get_client_with_logged_in_admin();
  let student: Value = common::create_test_student(&client);

  // Test
  let response = client
    .put(format!("{}/students/{}", common::APP_HOST, student["id"]))
    .json(&common::create_json_student())
    .send()
    .unwrap();

  assert_eq!(response.status(), StatusCode::OK);
  let student: Value = response.json().unwrap();
  assert_eq!(student, common::equal_data_student(student.clone()));
  
  // Cleanup
  common::delete_test_student(&client, student);
}

#[test]
fn test_delete_student() {
  let client = common::get_client_with_logged_in_admin();
  let student: Value = common::create_test_student(&client);

  let response = client
    .delete(format!("{}/students/{}", common::APP_HOST, student["id"]))
    .send()
    .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}