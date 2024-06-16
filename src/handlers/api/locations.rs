use axum::{
  extract::{Path, Query},
  response::{Html, IntoResponse},
  routing::get,
  Json, Router,
};
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::{json, Value};

// use crate::models::query::LocationsByFiltersParams;

#[derive(Debug, Deserialize)]
struct LocationsByFiltersParams {
  aor: Option<i32>,
  segment: Option<i32>,
  segments: Option<Vec<i32>>,
  customer: Option<i32>,
  region: Option<i32>,
  family: Option<i32>,
  #[serde(rename = "startHour")]
  start_hour: Option<i32>,
  #[serde(rename = "endHour")]
  end_hour: Option<i32>,
  parameters: Option<String>,
  shifts: Option<String>,
}

pub fn routes() -> Router {
  Router::new()
    .route(
      "/locations/data/byfpsid/:ids",
      get(handle_locations_by_fpsid),
    )
    .route("/locations/data/byfilters", get(handle_locations_by_filter))
}

async fn handle_locations_by_fpsid(Path(ids): Path<String>) -> (StatusCode, Json<Value>) {
  println!("->> {:<12} - handle_locations_by_fpsid {ids:?}", "HANDLER");
  let parsed: Vec<&str> = ids.split(",").collect();
  let id_ints: Vec<i32> = parsed.iter().flat_map(|x| x.parse()).collect();

  let body = Json(json!({
      "result": {
          "user": {
              "name": format!("handle_locations_by_fpsid {id_ints:?}")
          }
      }
  }));

  (StatusCode::OK, body)
}

async fn handle_locations_by_filter(
  params: Query<LocationsByFiltersParams>,
) -> (StatusCode, Json<Value>) {
  println!(
    "->> {:<12} - handler_locations_by_filter {params:?}",
    "HANDLER"
  );

  let body = Json(json!({
      "result": {
          "user": {
              "name": "handle_locations_by_filter"
          }
      }
  }));
  (StatusCode::OK, body)
}
