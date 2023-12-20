
use std::fmt::Display;

use ledger::Ledger;

pub mod account;
pub mod exit;

pub enum CmdError {
    Syntax(String)
}

pub enum CmdResult {
    Ok, 
    SignalTerminate
}

impl Display for CmdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CmdError::Syntax(msg) => write!(f, "{}", msg),
        }
    }
}

// Base for all commands
pub trait Cmd {
    fn new() -> Self where Self: Sized;
    fn execute(&self, args: Vec<&str>, ledger: &mut Ledger) -> Result<CmdResult, CmdError>;
    fn names(&self) -> Vec<&'static str>;
}