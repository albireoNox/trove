//! The "Ledger" type represents the user's total financial state, including transactions, accounts, transaction categories, etc.
//! It is the top-level object the application interacts with in order to query and manipulate user data. 

pub mod account;
pub mod common_types;
pub mod transaction;

use account::Account;

// TODO: Move this to it's own file, if it's not annoying from a module hierarchy standpoint. 
#[derive(serde::Deserialize, serde::Serialize)]
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

    pub fn get_accounts(&self) -> &Vec<Account> {
        &self.accounts
    }

    pub fn get_account_by_name_mut(&mut self, name: &String) -> Option<&mut Account> {
        for account in self.accounts.iter_mut() {
            if account.get_name().eq_ignore_ascii_case(name) {
                return Some(account);
            }
        }

        return None
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

    #[test]
    fn get_account_by_name_success() {
        let mut ledger = Ledger::new_empty();
        let name = "My Account".to_string();
        ledger.add_new_account(name.clone());
        assert_eq!(ledger.get_account_by_name_mut(&name).expect("FAILURE").get_name(), &name);
    }

    #[test]
    fn get_account_by_name_not_found() {
        let mut ledger = Ledger::new_empty();
        let name = "My Account".to_string();
        ledger.add_new_account(name.clone());
        assert!(ledger.get_account_by_name_mut(&"INVALID ACCOUNT".to_string()).is_none());
    }

    #[test]
    fn get_account_by_name_empty() {
        let mut ledger = Ledger::new_empty();
        assert!(ledger.get_account_by_name_mut(&"My Account".to_string()).is_none());
    }
}