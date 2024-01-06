use chrono::{DateTime, Utc};
use serde::Deserialize;
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

    pub fn get_amount(&self) -> &Money {
        &self.amount
    }
}

// Chrono::DateTime does not implement serde serialization, so we need to handle that here. 

impl serde::Serialize for Transaction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer 
    {
        let data = (self.amount, self.time.timestamp(), &self.description);
        data.serialize(serializer)
    }
}

impl<'a> serde::Deserialize<'a> for Transaction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'a> 
    {
        let data: (Money, i64, String) = Deserialize::deserialize(deserializer)?;
        // TODO don't just fail here, do something useful
        let time = DateTime::<Utc>::from_timestamp(data.1, 0).expect("Invalid timestamp found");
        Ok(Transaction { amount: data.0, time, description: data.2 })
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