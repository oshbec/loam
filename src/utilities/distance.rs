pub struct Distance(f64);

const METERS_PER_KILOMETER: f64 = 1000.0;
const METERS_PER_MILE: f64 = 1609.344;

impl Distance {
    pub fn from_meters(meters: f64) -> Distance {
        Distance(meters)
    }

    pub fn from_kilometers(kilometers: f64) -> Distance {
        Distance(kilometers * METERS_PER_KILOMETER)
    }

    pub fn from_miles(miles: f64) -> Distance {
        Distance(miles * METERS_PER_MILE)
    }

    pub fn in_meters(&self) -> f64 {
        self.0
    }

    pub fn in_kilometers(&self) -> f64 {
        self.0 / METERS_PER_KILOMETER
    }

    pub fn in_miles(&self) -> f64 {
        self.0 / METERS_PER_MILE
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    const MILES_PER_KILOMETER: f64 = 0.621_371_192_237_334;

    #[test]
    fn converts_to_stuff() {
        let distance = Distance::from_meters(METERS_PER_KILOMETER);
        assert_relative_eq!(distance.in_kilometers(), 1.0);
        assert_relative_eq!(distance.in_miles(), MILES_PER_KILOMETER);
    }

    #[test]
    fn converts_from_kilometers() {
        let distance = Distance::from_kilometers(1.0);
        assert_relative_eq!(distance.in_meters(), METERS_PER_KILOMETER);
    }

    #[test]
    fn converts_from_miles() {
        let distance = Distance::from_miles(1.0);
        assert_relative_eq!(distance.in_meters(), METERS_PER_MILE);
    }
}
