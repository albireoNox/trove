#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
pub struct Money {
    // For now assume it's USD and just store the number of cents
    // Effective range is ~ ±$92 trillion
    cents: i64,
}


impl Money {
    pub fn new(cents: i64) -> Money {
        Money {cents}
    }

    /// Create a money object from a float representing number of dollars with cents as the decimal part, e.g. 199.99
    /// Using this could potentially result in floating-point errors for large numbers
    pub fn from_float(val: f64) -> Money {
        Money {cents: (val * 100.0).round() as i64}
    }
}

impl std::fmt::Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}${:.2}", 
            if self.cents < 0 {"-"} else {""},
            (self.cents as f64).abs() / 100.0)
    }
}

impl<'a> std::iter::Sum<&'a Money> for Money {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        let mut sum = Money::from_float(0.0);
        for m in iter {
            sum += *m;
        }
        sum
    }
}

impl std::ops::Add for Money {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Money::new(self.cents + rhs.cents)
    }
}

impl std::ops::AddAssign for Money {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
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

    #[test]
    fn add() {
        assert_eq!(Money::from_float(10.00) + Money::from_float(0.50), Money::from_float(10.50))
    }

    #[test]
    fn add_assign() {
        let mut m = Money::from_float(10.00);
        m += Money::from_float(0.50);
        assert_eq!(m, Money::from_float(10.50))
    }

    #[test]
    fn sum() {
        let a = vec![
            Money::from_float(1.00),
            Money::from_float(0.50),
            Money::from_float(100.00)];
        let s: Money = a.iter().sum();
        assert_eq!(s, Money::from_float(101.50))
    }
}
