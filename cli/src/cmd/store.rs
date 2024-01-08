use ledger::Ledger;

use crate::{cmd::CmdResult, application::Application};

use super::CmdError;

/// Command to save user data
pub struct Store {
}

impl super::Cmd for Store {
    fn new() -> Store {
        Store{}
    }

    fn execute(&self, _args: Vec<&str>, ledger: &mut Ledger, app: &mut Application) -> Result<CmdResult, CmdError> {
        println!("Saving user data...");
        if let Err(e) = app.file_store.store_ledger(ledger) {
            eprintln!("Failed to save data!");
            return Err(CmdError::Dependency(e))
        }
        println!("Saved!");
        Ok(CmdResult::Ok)
    }

    fn names(&self) -> Vec<&'static str> {
        vec!["store", "save"]
    }
}