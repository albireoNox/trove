use std::fmt;

#[derive(PartialEq, PartialOrd, Debug)]
pub struct Money {
    // For now assume it's USD and just store the number of cents
    // Effective range is ~ ±$92 trillion
    cents: i64,
}


impl Money {
    fn new(cents: i64) -> Money {
        Money {cents}
    }

    /// Create a money object from a float representing number of dollars with cents as the decimal part, e.g. 199.99
    /// Using this could potentially result in floating-point errors for numbers with a large number of significant figures
    fn from_float(val: f64) -> Money {
        Money {cents: (val * 100.0).round() as i64}
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}${:.2}", 
            if self.cents < 0 {"-"} else {""},
            (self.cents as f64).abs() / 100.0)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        assert_eq!(Money::new(100), Money{cents: 100});
    }

    #[test]
    fn from_float() {
        assert_eq!(Money::from_float(123.45), Money{cents: 12345});
    }

    #[test]
    fn from_float_negative() {
        assert_eq!(Money::from_float(-123.45), Money{cents: -12345});
    }

    #[test]
    fn from_float_zero() {
        assert_eq!(Money::from_float(0.0), Money{cents: 0});
    }

    #[test]
    fn from_float_fractional_cents() {
        // round up
        assert_eq!(Money::from_float(666.666), Money{cents: 66667});
        // round down
        assert_eq!(Money::from_float(11.111), Money{cents: 1111});
    }

    #[test]
    fn fmt_basic() {
        assert_eq!(format!("{}", Money::from_float(1.23)), "$1.23")
    }

    #[test]
    fn fmt_whole_dollar_amount() {
        assert_eq!(format!("{}", Money::from_float(10.0)), "$10.00")
    }

    #[test]
    fn fmt_less_than_1() {
        assert_eq!(format!("{}", Money::from_float(0.01)), "$0.01")
    }

    #[test]
    fn fmt_negative() {
        assert_eq!(format!("{}", Money::from_float(-1.23)), "-$1.23")
    }

    #[test]
    fn fmt_zero() {
        assert_eq!(format!("{}", Money::from_float(0.0)), "$0.00")
    }

}
