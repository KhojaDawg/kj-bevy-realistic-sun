//! Some constants used for unit conversion
use std::f32::consts::TAU;


pub const DEG_TO_RAD: f32 = TAU / 360.0;
pub const HOURS_TO_RAD: f32 = TAU / 24.0;
// pub const RAD_TO_DEG: f32 = 360.0 / TAU;
// pub const RAD_TO_HOURS: f32 = 24.0 / TAU;


#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;
    use approx::ulps_eq;

    #[test]
    fn deg_to_rad() {
        let tests = vec![
            (0.0, 0.0),
            (PI/2.0, 90.0),
            (PI, 180.0),
            (TAU, 360.0),
        ];
        for (expected, input) in tests {
            let result = input * DEG_TO_RAD;
            assert!(
                ulps_eq!(result, expected),
                "Expected {} to convert to {}, but {} was computed", input, expected, result,
            );
        }
    }
}
