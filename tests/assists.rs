use reqwest::{blocking::Client, StatusCode};
use rocket::serde::json::{serde_json::json, Value};

pub mod common;

#[test]
fn test_get_assists() {
  // Setup
  let client = Client::new();
  let presence1 = common::create_test_presence(&client);
  let presence2 = common::create_test_presence(&client);

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
}

#[test]
fn test_create_presence() {
  // Setup
  let client = Client::new();
  let response = client
    .post(format!("{}/assists", common::APP_HOST))
    .json(&common::create_json_presence())
    .send()
    .unwrap();
  assert_eq!(response.status(), StatusCode::CREATED);

  // Test
  let presence: Value = response.json().unwrap();
  assert_eq!(presence, common::equal_data_presence(presence.clone()));

  // Cleanup
  common::delete_test_presence(&client, presence);
}

#[test]
fn test_view_presence() {
  // Setup
  let client = Client::new();
  let presence: Value = common::create_test_presence(&client);

  // Test
  let response = client
    .get(format!("{}/assists/{}", common::APP_HOST, presence["id"]))
    .send()
    .unwrap();

  assert_eq!(response.status(), StatusCode::OK);
  let presence: Value = response.json().unwrap();
  assert_eq!(presence, common::equal_data_presence(presence.clone()));

  // Cleanup
  common::delete_test_presence(&client, presence);
}

#[test]
fn test_update_presence() {
  // Setup
  let client = Client::new();
  let presence: Value = common::create_test_presence(&client);

  // Test
  let response = client
    .put(format!("{}/assists/{}", common::APP_HOST, presence["id"]))
    .json(&json!({
      "students_id": 1,
      "presence": "2023-06-18T09:22:22.731"
    }))
    .send()
    .unwrap();

  assert_eq!(response.status(), StatusCode::OK);
  let presence: Value = response.json().unwrap();
  assert_eq!(presence, common::equal_data_presence(presence.clone()));
  
  // Cleanup
  common::delete_test_presence(&client, presence);
}

#[test]
fn test_delete_presence() {
  let client = Client::new();
  let presence: Value = common::create_test_presence(&client);

  let response = client
    .delete(format!("{}/assists/{}", common::APP_HOST, presence["id"]))
    .send()
    .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}