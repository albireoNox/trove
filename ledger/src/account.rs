use crate::transaction::Transaction;

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
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_empty() {
        let account = Account::new_empty("Hello checking".to_string());
        assert_eq!(account.transactions.len(), 0);
        assert_eq!(account.name, "Hello checking")
    }
}