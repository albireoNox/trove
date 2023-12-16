use crate::application::Application;


pub struct Exit {
}

impl Exit {
    pub fn new() -> Exit {
        Exit{}
    }
}

impl super::Cmd for Exit {
    fn execute(&self, _args: Vec<&str>, application: &dyn Application) -> super::CmdResult {
        println!("Exiting...");
        application.signal_terminate();
        Ok(())
    }

    fn names(&self) -> Vec<&'static str> {
        vec!["exit", "ex", "quit"]
    }
}