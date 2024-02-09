
use std::{error::Error, fmt::Display};

use ledger::Ledger;
use super::app::Application;

pub mod account;
pub mod exit;
pub mod load;
pub mod store;
pub mod transaction;

#[derive(Debug)]
pub struct CmdError {    
    pub cmd_name: Option<String>,
    pub error_type: CmdErrorType
}

static DEFAULT_NAME: &str = "<unknown>";

#[derive(Debug)]
pub enum CmdErrorType {
    Syntax(SyntaxErrorType),
    Argument(String),
    Dependency(Box<dyn Error>),
    InvalidCommand(String)
}

#[derive(Debug)]
pub enum SyntaxErrorType {
    MissingSubcommand, 
    InvalidSubcommand(String),
    MissingParam(String),
}

impl From<std::io::Error> for CmdError {
    fn from(e: std::io::Error) -> Self {
        CmdError {
            cmd_name: None, 
            error_type: CmdErrorType::Dependency(Box::new(e))
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum CmdResult {
    Ok, 
    SignalTerminate
}

impl Display for CmdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name: &str = self.cmd_name
            .as_deref()
            .unwrap_or(DEFAULT_NAME);
        match &self.error_type {
            CmdErrorType::Syntax(SyntaxErrorType::MissingSubcommand) => {
                write!(f, "Command '{}' needs subcommand.", name)
            },
            CmdErrorType::Syntax(SyntaxErrorType::InvalidSubcommand(invalid_cmd)) => {
                write!(
                    f,
                    "Inavlid subcommand '{}' for command {}.", 
                    invalid_cmd,
                    name)
            },
            CmdErrorType::Syntax(SyntaxErrorType::MissingParam(msg)) => {
                write!(f, "{}", msg)
            }
            CmdErrorType::Argument(msg) => write!(f, "{}", msg),
            CmdErrorType::InvalidCommand(cmd) => write!(f, "Could not find command named '{}'", cmd),
            CmdErrorType::Dependency(err) => err.fmt(f),
        }
    }
}

impl Error for CmdError { }

// Base for all commands
pub trait Cmd {
    fn new() -> Self where Self: Sized;
    fn execute(&self, args: &[&str], ledger: &mut Ledger, app: &mut Application) -> Result<CmdResult, CmdError>;
    fn names(&self) -> Vec<&'static str>;
    fn help_text(&self) -> &'static str;

    fn primary_name(&self) -> &'static str {
        return self.names()[0];
    }

    fn new_error(&self, error_type: CmdErrorType) -> CmdError {
        CmdError { cmd_name: Some(self.primary_name().to_string()), error_type }
    }
}