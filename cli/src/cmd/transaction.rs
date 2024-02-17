use chrono::{DateTime, Utc};
use ledger::{account::Account, common_types::Money};

use crate::app::Application;

use super::{Cmd, CmdErrorType, CmdResult, SyntaxErrorType};

pub struct Transaction { }

impl Cmd for Transaction {
    fn new() -> Self where Self: Sized {
        Transaction {  }
    }

    fn execute(&self, args: &[&str], ledger: &mut ledger::Ledger, _app: &mut Application) -> Result<super::CmdResult, super::CmdError> {
        if args.len() < 3 {
            return Err(self.new_error(
                CmdErrorType::Syntax(SyntaxErrorType::MissingParam(
                    "Invalid format. Usage: `transaction [account_name] [amount] [description]`".to_string()))))
        }

        let account_name = args[0].to_string();
        let amount: f64 = args[1].parse().map_err(|e| self.new_error(CmdErrorType::Dependency(Box::new(e))))?;
        let description = args[2].to_string();

        let category_id = match args.get(3) {
            Some(s) => {
                let id = String::from(*s).into();
                let category = ledger.get_transaction_categories().get_category(&id);
                if category.is_none() {
                    return Err(self.new_error(
                        CmdErrorType::Argument(
                            format!("No category named '{}'", id)
                        )))
                }
                Some(id)
            },
            None => None,
        };

        let account: &mut Account = ledger.get_account_by_name_mut(&account_name).ok_or(
            self.new_error(CmdErrorType::Argument(format!("Could not find account named '{}'", account_name))))?;

        // TODO: ummm...get this from somewhere
        let time: DateTime<Utc> = Utc::now();

        // TODO: get category
        account.add_transaction(ledger::transaction::Transaction::new(Money::from_float(amount), time, description, category_id));

        Ok(CmdResult::Ok)
    }

    fn names(&self) -> Vec<&'static str> {
        vec!["transaction", "tr"]
    }

    fn help_text(&self) -> &'static str {
"Usage: transaction ACCOUNT AMOUNT DESCRIPTION [CATEGORY]
Creates a new transaction entry in ACCOUNT. 
"
    }
}