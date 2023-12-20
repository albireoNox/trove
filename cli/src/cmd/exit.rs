use ledger::Ledger;

use crate::cmd::CmdResult;

use super::CmdError;

/// Command to exit the program. 
pub struct Exit {
}

impl super::Cmd for Exit {
    fn new() -> Exit {
        Exit{}
    }

    fn execute(&self, _args: Vec<&str>, _ledger: &mut Ledger) -> Result<CmdResult, CmdError> {
        println!("Exiting...");
        Ok(CmdResult::SignalTerminate)
    }

    fn names(&self) -> Vec<&'static str> {
        vec!["exit", "ex", "quit"]
    }
}