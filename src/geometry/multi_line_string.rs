use super::prelude::*;

use serde::{Deserialize, Serialize};
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct MultiLineString {
    coordinates: Vec<Vec<Position>>,
}

impl Geometry for MultiLineString {}
