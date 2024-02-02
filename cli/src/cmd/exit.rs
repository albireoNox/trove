use ledger::Ledger;

use crate::{cmd::CmdResult, application::Application};

use super::CmdError;

/// Command to exit the program. 
pub struct Exit {
}

impl super::Cmd for Exit {
    fn new() -> Exit {
        Exit{}
    }

    fn execute(&self, _args: &[&str], _ledger: &mut Ledger, app: &mut Application) -> Result<CmdResult, CmdError> {
        writeln!(app.out(), "Exiting...")?;
        Ok(CmdResult::SignalTerminate)
    }

    fn names(&self) -> Vec<&'static str> {
        vec!["exit", "ex", "quit"]
    }

    fn help_text(&self) -> &'static str {
"Usage: exit
Exits the application."
    }
}