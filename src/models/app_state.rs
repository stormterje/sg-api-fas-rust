use axum::extract::FromRef;

use crate::{repos::filters_repo::FiltersRepo, services::filters_service::FiltersService};

#[derive(Clone, FromRef)]
pub struct AppState {
  pub filters_service: FiltersService,
}
