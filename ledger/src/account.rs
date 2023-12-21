use crate::{transaction::Transaction, common_types::Money};

pub struct Account {
    // For now, there's just a list of transactions. TODO: make this be not stupid. 
    transactions: Vec<Transaction>,
    name: String
}

impl Account {
    pub fn new_empty(name: String) -> Account {
        Account {name, transactions: Vec::new()}
    }   

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_total(&self) -> Money {
        self.transactions.iter().map(|t| t.get_amount()).sum()
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }
}
#[cfg(test)]
mod tests {
    use chrono::{DateTime, Utc};

    use super::*;

    fn test_transaction(amount: f64) -> Transaction {
        let amount = Money::from_float(amount);
        let time = "2000-1-1T00:00:00Z".parse::<DateTime<Utc>>().expect("Failed to parse");
        let description = "Widgets Inc.".to_string();

        Transaction::new(amount, time, description)
    }

    #[test]
    fn new_empty() {
        let account = Account::new_empty("Hello checking".to_string());
        assert_eq!(account.transactions.len(), 0);
        assert_eq!(account.name, "Hello checking")
    }

    #[test]
    fn add_transaction() {
        let mut account = Account::new_empty("Hello checking".to_string());
        account.add_transaction(test_transaction(100.0));
        assert_eq!(account.transactions.len(), 1);
    }

    #[test]
    fn get_total_empty() {
        let account = Account::new_empty("Hello checking".to_string());
        assert_eq!(account.get_total(), Money::from_float(0.0))
    }

    #[test]
    fn get_total() {
        let mut account = Account::new_empty("Hello checking".to_string());
        account.add_transaction(test_transaction(100.0));
        account.add_transaction(test_transaction(-50.0));
        account.add_transaction(test_transaction(0.01));
        assert_eq!(account.get_total(), Money::from_float(50.01))
    }
}