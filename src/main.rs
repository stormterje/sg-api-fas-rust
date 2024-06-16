#![allow(unused)]
mod handlers;
mod models;
mod repos;
mod services;

use std::collections::HashMap;

use axum::middleware;
use axum::routing::get_service;
use axum::{response::Response, Router};
use repos::filters_repo::FiltersRepo;
use services::filters_service::FiltersService;
use tokio::net::TcpListener;

use models::app_state::{self, AppState};

use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> Result<(), ()> {
  let filters_repo = FiltersRepo::new().await?;

  let app_state = AppState {
    filters_service: FiltersService::new(filters_repo).await?,
  };

  let routes = Router::new()
    .merge(handlers::api::routes(app_state))
    .layer(middleware::map_response(main_response_mapper))
    .fallback_service(static_routes());

  let listener = TcpListener::bind("127.0.0.1:5000").await.unwrap();
  println!("->> LISTENING on {:?}\n", listener.local_addr());

  axum::serve(listener, routes.into_make_service())
    .await
    .unwrap();

  Ok(())
}

async fn main_response_mapper(res: Response) -> Response {
  println!("->> {:<12} - main_response_mapper\n", "RES_MAPPER");
  res
}

fn static_routes() -> Router {
  Router::new().nest_service("/", get_service(ServeDir::new("./html")))
}
