use super::prelude::*;

pub struct GeometryCollection {
    pub geometries: Vec<Box<dyn Geometry>>,
}
