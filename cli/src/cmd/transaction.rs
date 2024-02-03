use chrono::{DateTime, Utc};
use ledger::{common_types::Money, account::Account};

use crate::app::Application;

use super::{Cmd, CmdError, CmdResult};

pub struct Transaction { }

impl Cmd for Transaction {
    fn new() -> Self where Self: Sized {
        Transaction {  }
    }

    fn execute(&self, args: &[&str], ledger: &mut ledger::Ledger, _app: &mut Application) -> Result<super::CmdResult, super::CmdError> {
        if args.len() != 3 {
            return Err(CmdError::Syntax("Invalid format. Usage: `transaction [account_name] [amount] [description]`".to_string()))
        }

        let account_name = args[0].to_string();
        let amount: f64 = args[1].parse().map_err(|e| CmdError::Dependency(Box::new(e)))?;
        let description = args[2].to_string();

        let account: &mut Account = ledger.get_account_by_name_mut(&account_name).ok_or(
            CmdError::Argument(format!("Could not find account named '{}'", account_name)))?;

        // TODO: ummm...get this from somewhere
        let time: DateTime<Utc> = Utc::now();

        account.add_transaction(ledger::transaction::Transaction::new(Money::from_float(amount), time, description));

        Ok(CmdResult::Ok)
    }

    fn names(&self) -> Vec<&'static str> {
        vec!["transaction", "tr"]
    }

    fn help_text(&self) -> &'static str {
"Usage: transaction ACCOUNT AMOUNT DESCRIPTION
Creates a new transaction entry in ACCOUNT. 
"
    }
}