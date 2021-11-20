//! Various [`f64`] and [`f32`] extensions

/// Trait that provides a way to round floats to a specific amount of decimals
pub trait RoundTo: Sized {
    /// Returns the nearest number to `self` rounded to `decimal`
    /// number of decimals. Half-way cases round away from 0.0.
    fn round_to(self, decimals: i32) -> Self;
}

impl RoundTo for f64 {
    fn round_to(self, decimals: i32) -> Self {
        let rounding_coefficient = (10.0_f64).powi(decimals);
        (self * rounding_coefficient).round() / rounding_coefficient
    }
}

impl RoundTo for f32 {
    fn round_to(self, decimals: i32) -> Self {
        let rounding_coefficient = (10.0_f32).powi(decimals);
        (self * rounding_coefficient).round() / rounding_coefficient
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_eq::assert_float_eq;

    #[test]
    fn round_to_tests() {
        let value = 1.23456789;

        assert_float_eq!(value.round_to(1), 1.2, abs <= 0.1);
        assert_float_eq!(value.round_to(2), 1.23, abs <= 0.01);
        assert_float_eq!(value.round_to(3), 1.235, abs <= 0.001);
        assert_float_eq!(value.round_to(4), 1.234_6, abs <= 0.000_1);
        assert_float_eq!(value.round_to(5), 1.234_57, abs <= 0.000_01);
        assert_float_eq!(value.round_to(6), 1.234_568, abs <= 0.000_001);
        assert_float_eq!(value.round_to(7), 1.234_567_9, abs <= 0.000_000_1);
        assert_float_eq!(value.round_to(8), 1.234_567_89, abs <= 0.000_000_01);
    }
}
