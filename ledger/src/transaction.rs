use chrono::{DateTime, Utc};
use serde::{de, Deserialize};
use crate::category::CategoryId;
use super::common_types::Money;

#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Transaction {
    amount: Money,
    time: Timestamp,
    description: String, 
    category: Option<CategoryId>,
}

impl Transaction {
    pub fn new(
        amount: Money, 
        time: DateTime<Utc>, 
        description: String, 
        category: Option<CategoryId>,
    ) -> Transaction {
        Transaction {amount, time: Timestamp::from(time), description, category}
    }

    pub fn get_amount(&self) -> &Money {
        &self.amount
    }
}

#[derive(Debug, PartialEq)]
struct Timestamp(DateTime<Utc>);

impl From<DateTime<Utc>> for Timestamp {
    fn from(value: DateTime<Utc>) -> Self {
        Timestamp(value)
    }
}

impl serde::Serialize for Timestamp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer 
    {
        self.0.timestamp().serialize(serializer)    
    }
}

impl<'a> serde::Deserialize<'a> for Timestamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'a> 
    {
            let data: i64 = Deserialize::deserialize(deserializer)?;
            DateTime::<Utc>::from_timestamp(data, 0)
                .map(Timestamp::from)
                .ok_or_else(|| de::Error::custom("invalid timestamp found"))
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
            Transaction::new(amount, time, description.clone(), None),
            Transaction {amount, time: Timestamp(time), description, category: None});
    }
}