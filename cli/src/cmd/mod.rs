use std::error::Error;

use crate::application::Application;

pub mod exit;

type CmdResult = Result<(), Box<dyn Error>>;

pub trait Cmd {
    fn execute(&self, args: Vec<&str>, application: &dyn Application) -> CmdResult;
    fn names(&self) -> Vec<&'static str>;
}