use crate::geometry::prelude::*;
use crate::utilities::prelude::*;

/// Calculates the distance between two positions on the Earth's surface using the Haversine formula.
///
/// The Haversine formula is used to calculate the great-circle distance between two points on a sphere given their longitudes and latitudes.
/// This formula takes into account the curvature of the Earth and provides a more accurate distance measurement compared to simple Euclidean distance calculations.
///
/// # Arguments
///
/// * `from` - The starting position.
/// * `to` - The ending position.
///
/// # Returns
///
/// The distance between the two positions in meters.
///
/// # Examples
///
/// ```
/// use loam::measurement::distance::*;
/// use loam::geometry::prelude::*;
///
/// let position1 = Position::from((42.232, -83.767));
/// let position2 = Position::from((40.669, -89.588));
/// let calculated_distance = haversine(position1, position2);
///
/// assert_eq!(calculated_distance.in_meters(), 647_284.808_500_878_4);
/// ```
pub fn haversine(from: Position, to: Position) -> Distance {
    let radius_earth = 6_371_008.8;

    let lat_from = from.latitude.to_radians();
    let lat_to = to.latitude.to_radians();

    let delta_lat = (to.latitude - from.latitude).to_radians();
    let delta_lon = (to.longitude - from.longitude).to_radians();

    let a = (delta_lat / 2.0).sin().powi(2)
        + lat_from.cos() * lat_to.cos() * (delta_lon / 2.0).sin().powi(2);

    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    Distance::from_meters(radius_earth * c)
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn from_costco_to_costco() {
        let costco_1 = Position::from((42.232, -83.767));
        let costco_2 = Position::from((40.669, -89.588));
        let calculated_distance = haversine(costco_1, costco_2);

        assert_relative_eq!(calculated_distance.in_meters(), 647_284.808_500_878_4);
    }
}
