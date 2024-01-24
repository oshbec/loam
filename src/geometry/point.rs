use serde::{Deserialize, Serialize};

use super::prelude::*;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point {
    coordinates: Position,
}

impl Geometry for Point {}

impl From<Position> for Point {
    fn from(position: Position) -> Self {
        Point {
            coordinates: position,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_a_point() {
        let point: Point = Position::from((1.0, 2.0)).into();
        let serialized = serde_json::to_string(&point).unwrap();
        assert_eq!(serialized, r#"{"type":"Point","coordinates":[1.0,2.0]}"#);
    }

    #[test]
    fn deserialize_a_point() {
        let json = r#"{"type":"Point","coordinates":[1.0,2.0,3.0]}"#;
        let point: Point = serde_json::from_str(json).unwrap();
        let position: Position = (1.0, 2.0, 3.0).into();
        assert_eq!(point.coordinates, position);
    }
}
