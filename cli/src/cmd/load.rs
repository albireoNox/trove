use ledger::Ledger;

use crate::{cmd::CmdResult, application::Application};

use super::CmdError;

/// Command to load user data from disk
pub struct Load {
}

impl super::Cmd for Load {
    fn new() -> Load {
        Load{}
    }

    fn execute(&self, _args: Vec<&str>, ledger: &mut Ledger, app: &mut Application) -> Result<CmdResult, CmdError> {
        println!("Loading user data...");
        match app.file_store.load_ledger() {
            Ok(new_ledger) => {
                *ledger = new_ledger
            },
            Err(e) => {
                return Err(CmdError::Dependency(e))
            },
        }
        println!("Loaded!");
        Ok(CmdResult::Ok)
    }

    fn names(&self) -> Vec<&'static str> {
        vec!["load"]
    }
}