use chrono::{DateTime, Utc};
use ledger::{common_types::Money, account::Account};

use super::{Cmd, CmdError, CmdResult};

pub struct Transaction { }

impl Cmd for Transaction {
    fn new() -> Self where Self: Sized {
        Transaction {  }
    }

    fn execute(&self, args: Vec<&str>, ledger: &mut ledger::Ledger) -> Result<super::CmdResult, super::CmdError> {
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
        return vec!["transaction", "tr"]
    }
}