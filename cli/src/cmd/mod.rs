
use std::{fmt::Display, error::Error};

use ledger::Ledger;
use super::application::Application;

pub mod account;
pub mod exit;
pub mod load;
pub mod store;
pub mod transaction;

#[derive(Debug)]
pub enum CmdError {
    Syntax(String),
    Argument(String),
    Dependency(Box<dyn Error>),
}

#[derive(Debug)]
pub enum CmdResult {
    Ok, 
    SignalTerminate
}

impl Display for CmdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CmdError::Syntax(msg) => write!(f, "{}", msg),
            CmdError::Argument(msg) => write!(f, "{}", msg),
            CmdError::Dependency(err) => err.fmt(f),
        }
    }
}

impl Error for CmdError { }

// Base for all commands
pub trait Cmd {
    fn new() -> Self where Self: Sized;
    fn execute(&self, args: Vec<&str>, ledger: &mut Ledger, app: &mut Application) -> Result<CmdResult, CmdError>;
    fn names(&self) -> Vec<&'static str>;
}