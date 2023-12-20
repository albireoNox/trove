//! The "Ledger" type represents the user's total financial state, including transactions, accounts, transaction categories, etc.
//! It is the top-level object the application interacts with in order to query and manipulate user data. 

mod account;
mod common_types;
mod transaction;

use account::Account;

pub struct Ledger {
    accounts: Vec<Account>,
}

impl Ledger {
    pub fn new_empty() -> Ledger {
        Ledger {accounts: Vec::new()}
    }

    pub fn add_new_account(&mut self, name: String) {
        let new_account = Account::new_empty(name);
        self.accounts.push(new_account);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_empty() {
        let ledger = Ledger::new_empty();
        assert_eq!(ledger.accounts.len(), 0);
    }

    #[test]
    fn new_account() {
        let mut ledger = Ledger::new_empty();
        ledger.add_new_account("My Account".to_string());
        assert_eq!(ledger.accounts.len(), 1);
        assert_eq!(ledger.accounts[0].get_name(), "My Account");
    }
}