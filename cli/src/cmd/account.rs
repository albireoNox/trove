use ledger::Ledger;

use super::{Cmd, CmdError, CmdResult};


pub struct Account {
}

impl Cmd for Account {
    fn new() -> Account {
        Account { }
    }

    fn execute(&self, args: Vec<&str>, ledger: &mut Ledger) -> Result<CmdResult, CmdError> {
        match args.get(0) {
            Some(&"--new") => {
                self.add_new_account(&args[1..], ledger)
            },
            Some(&"--list") => {
                self.list_accounts(ledger)
            }
            Some(unhandled_subcmd) => {
                Err(CmdError::Syntax(format!("Subcommand '{}' not implemented for command 'account'", unhandled_subcmd)))
            }
            None => {
                Err(CmdError::Syntax("Command 'account' needs subcommand.".to_string()))
            },
        }
    }

    fn names(&self) -> Vec<&'static str> {
        vec!["account", "acc", "ac"]
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

    fn list_accounts(&self, ledger: &Ledger) -> Result<CmdResult, CmdError> {
        for account in ledger.get_accounts() {
            println!("  {}\t{}", account.get_name(), account.get_total())
        }

        Ok(CmdResult::Ok)
    }
}