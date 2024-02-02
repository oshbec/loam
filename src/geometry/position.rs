use serde::de::{self, Deserialize, Deserializer, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeTuple, Serializer};
use std::fmt;

impl From<(f64, f64)> for Position {
    fn from(coords: (f64, f64)) -> Self {
        Position {
            longitude: coords.0,
            latitude: coords.1,
            altitude: None,
        }
    }
}

impl From<(f64, f64, f64)> for Position {
    fn from(coords: (f64, f64, f64)) -> Self {
        Position {
            longitude: coords.0,
            latitude: coords.1,
            altitude: Some(coords.2),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Position {
    pub longitude: f64,
    pub latitude: f64,
    pub altitude: Option<f64>,
}

impl Serialize for Position {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_tuple(3)?;
        seq.serialize_element(&self.longitude)?;
        seq.serialize_element(&self.latitude)?;
        if let Some(altitude) = self.altitude {
            seq.serialize_element(&altitude)?;
        }
        seq.end()
    }
}

impl<'de> Deserialize<'de> for Position {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PositionVisitor;

        impl<'de> Visitor<'de> for PositionVisitor {
            type Value = Position;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an array of two or three elements")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Position, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let longitude: f64 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let latitude: f64 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let altitude: Option<f64> = seq.next_element()?;

                Ok(Position {
                    longitude,
                    latitude,
                    altitude,
                })
            }
        }

        deserializer.deserialize_seq(PositionVisitor)
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn construct_2d() {
        let position = Position::from((1.0, 2.0));
        assert_relative_eq!(position.longitude, 1.0);
        assert_relative_eq!(position.latitude, 2.0);
        assert_eq!(position.altitude, None);
    }

    #[test]
    fn construct_3d() {
        let position = Position::from((1.0, 2.0, 3.0));
        assert_relative_eq!(position.longitude, 1.0);
        assert_relative_eq!(position.latitude, 2.0);
        assert_relative_eq!(position.altitude.unwrap(), 3.0);
    }

    #[test]
    fn serializes_2d() {
        let position = Position::from((1.0, 2.0));
        let serialized = serde_json::to_string(&position).unwrap();
        assert_eq!(serialized, "[1.0,2.0]");
    }

    #[test]
    fn serializes_3d() {
        let position = Position::from((1.0, 2.0, 3.0));
        let serialized = serde_json::to_string(&position).unwrap();
        assert_eq!(serialized, "[1.0,2.0,3.0]");
    }

    #[test]
    fn deserializes_2d() {
        let json = "[1.0, 2.0]";
        let position: Position = serde_json::from_str(json).unwrap();
        assert_relative_eq!(position.longitude, 1.0);
        assert_relative_eq!(position.latitude, 2.0);
        assert_eq!(position.altitude, None);
    }

    #[test]
    fn deserializes_3d() {
        let json = "[1.0, 2.0, 3.0]";
        let position: Position = serde_json::from_str(json).unwrap();
        assert_relative_eq!(position.longitude, 1.0);
        assert_relative_eq!(position.latitude, 2.0);
        assert_eq!(position.altitude, Some(3.0));
    }

    #[test]
    fn wont_deserialize_empty_array() {
        let json = "[]"; // Empty array
        let result: Result<Position, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Deserializing an array with 0 elements should result in an error."
        );
    }

    #[test]
    fn wont_deserialize_1_element() {
        let json = "[1.0]"; // Array with only 1 element
        let result: Result<Position, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Deserializing an array with 1 element should result in an error."
        );
    }

    #[test]
    fn wont_deserialze_more_than_4_elements() {
        let json = "[1.0, 2.0, 3.0, 4.0]"; // Array with 4 elements
        let result: Result<Position, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Deserializing an array with 4 elements should result in an error."
        );
    }
}
