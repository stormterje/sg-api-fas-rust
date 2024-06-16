use std::collections::HashMap;

use axum::{
  extract::{Path, State},
  http::Error,
  response::{Html, IntoResponse},
  routing::{get, post},
  Json, Router,
};
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::{json, Value};
use tracing_subscriber::filter;

use crate::{
  models::{app_state::AppState, filters::Filters},
  services::filters_service::FiltersService,
};
pub fn routes(app_state: AppState) -> Router {
  Router::new()
    .route("/filters/:name", get(get_filters))
    .with_state(app_state.filters_service)
}

async fn get_filters(
  State(fs): State<FiltersService>,
  Path(name): Path<String>,
) -> (StatusCode, Json<Filters>) {
  println!("->> {:<12} - GET filters/{name}", "HANDLER");

  if name != "all" {
    return (StatusCode::BAD_REQUEST, Json(Filters::default()));
  }

  let filters = fs.get_filters().await;
  match filters {
    Ok(f) => (StatusCode::OK, Json(f)),
    Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(Filters::default())),
  }
}
