use super::prelude::*;

pub struct GeometryCollection {
    geometries: Vec<Box<dyn Geometry>>,
}
