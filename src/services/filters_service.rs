use crate::{models::filters::Filters, repos::filters_repo::FiltersRepo};
use anyhow::Error;
use serde::{Deserialize, Serialize};
use std::num::ParseIntError;
use std::{
  collections::HashMap,
  sync::{Arc, Mutex},
};

#[derive(Clone)]
pub struct FiltersService {
  filters_repo: FiltersRepo,
  filters_store: Arc<Mutex<Option<Filters>>>,
}

impl FiltersService {
  pub async fn new(repo: FiltersRepo) -> Result<Self, ()> {
    Ok(Self {
      filters_repo: repo,
      filters_store: Arc::default(),
    })
  }
}

impl FiltersService {
  pub async fn get_filters(&self) -> Result<Filters, Error> {
    println!("FiltersService::get_filters");
    let filters = reqwest::get("https://fps-api.stormgeo.com/fpsapi/v1/fas/filters.json")
      .await?
      .json::<Filters>()
      .await?;

    Ok(filters)
  }
}
