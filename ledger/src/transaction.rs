use chrono::{DateTime, Utc};
use super::common_types::Money;

#[derive(Debug, PartialEq)]
pub struct Transaction {
    amount: Money,
    time: DateTime<Utc>,
    description: String, 
}

impl Transaction {
    pub fn new(amount: Money, time: DateTime<Utc>, description: String) -> Transaction {
        Transaction {amount, time, description}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_empty() {
        let amount = Money::from_float(100.0);
        let time = "2000-1-1T00:00:00Z".parse::<DateTime<Utc>>().expect("Failed to parse");
        let description = "Widgets Inc.".to_string();

        assert_eq!(
            Transaction::new(amount, time, description.clone()),
            Transaction {amount, time, description});
    }
}