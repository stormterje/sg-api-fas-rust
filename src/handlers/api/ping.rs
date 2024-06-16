use anyhow::Error;
use axum::{response::IntoResponse, routing::get, Json, Router};
use reqwest::StatusCode;
use serde_json::{json, Value};

pub fn routes() -> Router {
  Router::new().route("/ping", get(handle_ping))
}

async fn handle_ping() -> (StatusCode, Json<Value>) {
  println!("->> {:<12} - handle_ping", "HANDLER");

  let body = Json(json!(
    {
      "result": {
        "message": "pong"
      }
    }
  ));

  (StatusCode::OK, body)
}
