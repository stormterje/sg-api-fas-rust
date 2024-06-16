use crate::models::filters::Filters;
use axum::Error;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct FiltersRepo {}

impl FiltersRepo {
  pub async fn new() -> Result<Self, ()> {
    Ok(Self {})
  }
}

impl FiltersRepo {
  pub async fn get_filters(&self) -> Result<Filters, Error> {
    Ok(Filters::default())
  }
}
