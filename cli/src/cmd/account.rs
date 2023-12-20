use ledger::Ledger;

use super::{Cmd, CmdError, CmdResult};


pub struct Account {
}

impl Account {
    pub fn new() -> Account {
        Account { }
    }
}

impl Cmd for Account {
    fn execute(&self, args: Vec<&str>, ledger: &mut Ledger) -> Result<CmdResult, CmdError> {
        match args.get(0) {
            Some(&"--new") => {
                self.add_new_account(&args[1..], ledger)
            },
            Some(unhandled_subcmd) => {
                return Err(CmdError::Syntax(format!("Subcommand '{}' not implemented for command 'account'", unhandled_subcmd)))
            }
            None => {
                return Err(CmdError::Syntax("Command 'account' needs subcommand.".to_string()))
            },
        }
    }

    fn names(&self) -> Vec<&'static str> {
        vec!["account", "acc"]
    }
}

impl Account {

    fn add_new_account(&self, args: &[&str], ledger: &mut Ledger) -> Result<CmdResult, CmdError> {
        let name = args.get(0).ok_or(
            CmdError::Syntax("Adding a new account requires an name".to_string()))?;

        ledger.add_new_account(String::from(*name));
        println!("Created account '{}'", name);
        Ok(CmdResult::Ok)
    }
}