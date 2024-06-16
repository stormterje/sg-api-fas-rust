use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Filters {
  pub aors: Option<Vec<Aor>>,
  pub segments: Option<Vec<Segmment>>,
  pub customers: Option<Vec<Customer>>,
  pub regions: Option<Vec<Region>>,
  pub families: Option<Vec<Family>>,
  pub groups: Option<Vec<Group>>,
  pub parameters: Option<Vec<Parameter>>,
  pub shifts: Option<Vec<Shift>>,
}

impl Default for Filters {
  fn default() -> Filters {
    Filters {
      aors: Option::None,
      segments: Option::None,
      customers: Option::None,
      families: Option::None,
      groups: Option::None,
      parameters: Option::None,
      regions: Option::None,
      shifts: Option::None,
    }
  }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Aor {
  pub id: String,
  pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Segmment {
  pub id: String,
  pub name: String,
  #[serde(rename = "parentId")]
  pub parent_id: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Customer {
  pub id: String,
  pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Region {
  pub id: String,
  pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Family {
  pub id: String,
  pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FpsPoint {
  pub id: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Parameter {
  pub id: i32,
  pub name: String,
  pub units: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Group {
  pub id: i32,
  pub name: String,
  pub parameters: Vec<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Shift {
  pub name: String,
}
