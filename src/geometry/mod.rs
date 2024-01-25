mod geometry_collection;
mod line_string;
mod multi_line_string;
mod multi_point;
mod multi_polygon;
mod point;
mod polygon;
mod position;

pub mod prelude {
    pub use super::geometry_collection::*;
    pub use super::line_string::*;
    pub use super::multi_line_string::*;
    pub use super::multi_point::*;
    pub use super::multi_polygon::*;
    pub use super::point::*;
    pub use super::polygon::*;
    pub use super::position::*;
    pub use super::*;
}

pub trait Geometry {}
