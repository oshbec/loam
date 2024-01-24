use super::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct MultiPoint {
    coordinates: Vec<Position>,
}

impl Geometry for MultiPoint {}
