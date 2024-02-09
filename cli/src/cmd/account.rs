use ledger::Ledger;

use crate::app::Application;

use super::{Cmd, CmdError, CmdErrorType, CmdResult, SyntaxErrorType};


pub struct Account {
}

impl Cmd for Account {
    fn new() -> Account {
        Account { }
    }

    fn execute(&self, args: &[&str], ledger: &mut Ledger, app: &mut Application) -> Result<CmdResult, CmdError> {
        match args.first() {
            Some(&"--new") => {
                self.add_new_account(&args[1..], ledger, app)
            },
            Some(&"--list") => {
                self.list_accounts(ledger, app)
            }
            Some(unhandled_subcommand) => {
                Err(self.new_error(CmdErrorType::Syntax(SyntaxErrorType::InvalidSubcommand(unhandled_subcommand.to_string()))))
            }
            None => {
                Err(self.new_error(CmdErrorType::Syntax(SyntaxErrorType::MissingSubcommand)))
            }
        }
    }

    fn names(&self) -> Vec<&'static str> {
        vec!["account", "acc", "ac"]
    }

    fn help_text(&self) -> &'static str {
"Usage: account [OPTION] ACCOUNT_NAME
Perform operations on user accounts. 

Options:
  --new    Create a new account with ACCOUNT_NAME
  --list   List the existing accounts"
    }
}

impl Account {

    fn add_new_account(&self, args: &[&str], ledger: &mut Ledger, app: &mut Application) -> Result<CmdResult, CmdError> {
        let name = args.first().ok_or(
            self.new_error(
                CmdErrorType::Syntax(
                    SyntaxErrorType::MissingParam(
                        "Adding a new account requires an name".to_string()))))?;

        ledger.add_new_account(String::from(*name));
        writeln!(app.out(), "Created account '{}'", name)?;
        Ok(CmdResult::Ok)
    }

    fn list_accounts(&self, ledger: &Ledger, app: &mut Application) -> Result<CmdResult, CmdError> {
        for account in ledger.get_accounts() {
            writeln!(app.out(), "  {}\t{}", account.get_name(), account.get_total())?;
        }

        Ok(CmdResult::Ok)
    }
}