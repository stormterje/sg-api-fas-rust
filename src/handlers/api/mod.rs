use axum::Router;

use crate::models::app_state::{self, AppState};

mod auth;
mod filters;
mod locations;
mod ping;

pub fn routes(app_state: AppState) -> Router {
  let api_routers = Router::new()
    .merge(ping::routes())
    .merge(filters::routes(app_state))
    .merge(locations::routes())
    .merge(auth::routes());

  Router::new().nest("/api", api_routers)
}
