use serde::{Deserialize, Serialize};

use super::prelude::*;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct LineString {
    coordinates: Vec<Position>,
}

impl Geometry for LineString {}
