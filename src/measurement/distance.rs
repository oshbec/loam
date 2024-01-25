use crate::geometry::prelude::*;

pub enum Distance {
    Kilometers(f32),
    Miles(f32),
}

impl Distance {
    pub fn to_miles(&self) -> Distance {
        match self {
            Distance::Kilometers(km) => Distance::Miles(km * 0.621_371),
            Distance::Miles(mi) => Distance::Miles(*mi),
        }
    }
}

fn distance(from: Position, to: Position) -> Distance {
    let radius_earth_km = 6_371_008.8 / 1000.0;

    let lat_from = from.latitude.to_radians();
    let lat_to = to.latitude.to_radians();

    let delta_lat = (to.latitude - from.latitude).to_radians();
    let delta_lon = (to.longitude - from.longitude).to_radians();

    let a = (delta_lat / 2.0).sin().powi(2)
        + lat_from.cos() * lat_to.cos() * (delta_lon / 2.0).sin().powi(2);

    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    let distance_km = radius_earth_km * c;

    Distance::Kilometers(distance_km)
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn from_costco_to_costco() {
        let costco_1 = Position::from((42.232, -83.767));
        let costco_2 = Position::from((40.669, -89.588));
        let calculated_distance = distance(costco_1, costco_2);

        if let Distance::Kilometers(km) = calculated_distance {
            assert_relative_eq!(km, 647.28467);
        }
    }
}
