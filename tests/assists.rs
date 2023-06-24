use reqwest::{blocking::Client, StatusCode};
use serde_json::Value;

pub mod common;

#[test]
fn test_get_assists() {
  // Setup
  let client = Client::new();
  let student = common::create_test_student(&client.clone());
  let presence1 = common::create_test_presence(&client, student.clone());
  let presence2 = common::create_test_presence(&client, student.clone());

  // Test
  let response = client
    .get(format!("{}/assists", common::APP_HOST))
    .send()
    .unwrap();
  assert_eq!(response.status(), StatusCode::OK);

  let json: Value = response.json().unwrap();
  assert!(json.as_array().unwrap().contains(&presence1));
  assert!(json.as_array().unwrap().contains(&presence2));

  // Cleanup
  common::delete_test_presence(&client, presence1);
  common::delete_test_presence(&client, presence2);
  common::delete_test_student(&client, student);
}

#[test]
fn test_create_presence() {
  // Setup
  let client = Client::new();

  // Test
  let student = common::create_test_student(&client);
  let response = client
    .post(format!("{}/assists", common::APP_HOST))
    .json(&common::create_json_presence(student.clone()))
    .send()
    .unwrap();
  assert_eq!(response.status(), StatusCode::CREATED);

  let presence: Value = response.json().unwrap();
  assert_eq!(presence, common::equal_data_presence(presence.clone(), student.clone()));

  // Cleanup
  common::delete_test_presence(&client, presence);
  common::delete_test_student(&client, student);
}

#[test]
fn test_view_presence() {
  // Setup
  let client = Client::new();
  let student = common::create_test_student(&client.clone());
  let presence: Value = common::create_test_presence(&client, student.clone());

  // Test
  let response: reqwest::blocking::Response = client
    .get(format!("{}/assists/{}", common::APP_HOST, presence["id"]))
    .send()
    .unwrap();

  assert_eq!(response.status(), StatusCode::OK);
  let presence: Value = response.json().unwrap();
  assert_eq!(presence, common::equal_data_presence(presence.clone(), student.clone()));

  // Cleanup
  common::delete_test_presence(&client, presence);
  common::delete_test_student(&client, student);
}

#[test]
fn test_update_presence() {
  // Setup
  let client = Client::new();
  let student = common::create_test_student(&client.clone());
  let presence: Value = common::create_test_presence(&client, student.clone());

  // Test
  let response = client
    .put(format!("{}/assists/{}", common::APP_HOST, presence["id"]))
    .json(&common::equal_data_presence(presence, student.clone()))
    .send()
    .unwrap();

  assert_eq!(response.status(), StatusCode::OK);
  let presence: Value = response.json().unwrap();
  assert_eq!(presence, common::equal_data_presence(presence.clone(), student.clone()));
  
  // Cleanup
  common::delete_test_presence(&client, presence);
  common::delete_test_student(&client, student);
}

#[test]
fn test_delete_presence() {
  let client = Client::new();
  let student = common::create_test_student(&client.clone());
  let presence: Value = common::create_test_presence(&client, student.clone());

  let response = client
    .delete(format!("{}/assists/{}", common::APP_HOST, presence["id"]))
    .send()
    .unwrap();

  assert_eq!(response.status(), StatusCode::NO_CONTENT);

  common::delete_test_student(&client, student);
}