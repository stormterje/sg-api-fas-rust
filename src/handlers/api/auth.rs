use axum::{response::Response, routing::post, Json, Router};
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::{json, Value};

pub fn routes() -> Router {
  Router::new().route("/login", post(handle_login))
}

async fn handle_login(payload: Json<LoginPayload>) -> (StatusCode, Json<Value>) {
  let uname = payload.username.to_owned();
  println!("->> {:<12} - handle_login {uname}", "HANDLER");
  if payload.username != "user1" || payload.password != "password" {
    return (StatusCode::BAD_REQUEST, Json(Value::Null));
  }

  let body = Json(json!({
      "result": {
          "user": {
              "name": payload.username
          }
      }
  }));

  (StatusCode::OK, body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
  username: String,
  password: String,
}
