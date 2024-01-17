//! This crate builds into an executible for running the CLI application. 
 
// In general, code that could apply to different types of applications (GUI. 
// web, etc.) should go elsewhere. 

use std::{io, error::Error, collections::HashMap, rc::Rc};
use application::Application;
use cmd::{Cmd, CmdError, CmdResult};
use ledger::Ledger;

mod cmd;
mod application;

fn main() -> Result<(), Box<dyn Error>> {
    let mut cli_app = CliApp::new(command_list());
    if let Err(e) = cli_app.run() {
        eprintln!("Encountered fatal error: {e}");
        eprintln!("Exiting...");

        return Err(e);
    }

    Ok(())
}

/// State used by the CLI application. Manages the top-level REPL loop and parses input
/// to dispatch to command structs. 
struct CliApp {
    cmd_map: HashMap<&'static str, Rc<dyn Cmd>>,
    ledger: Ledger,
    application: Application,
}

impl CliApp {
    fn new(cmds: Vec<Rc<dyn Cmd>>) -> CliApp {
        let application = Application::new_default();

        let mut cmd_map = HashMap::new();
        for cmd in &cmds {
            for name in cmd.names() {
                cmd_map.insert(name, cmd.clone());
            }
        }

        CliApp {
            cmd_map: cmd_map,
            ledger: Ledger::new_empty(), // TODO: load exiting one
            application: application
        }
    }

    // This function is not easily testible, since it reads from stdin. TODO: consider refactoring this out somehow. 
    fn run(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            let mut raw_input = String::new();
            io::stdin().read_line(&mut raw_input)?;

            match self.run_cmd(&raw_input) {
                Ok(CmdResult::SignalTerminate) => { 
                    break; 
                }, 
                Ok(CmdResult::Ok) => { /* Next command */ }
                // For now all errors are recoverable
                Err(e) => { 
                    eprintln!("{}", e); 
                },
            }
        }

        Ok(())
    }

    fn run_cmd(&mut self, raw_input: &String) -> Result<CmdResult, Box<dyn Error>> {
        let tokens = tokenize_string(raw_input);
    
        if tokens.len() == 0 {
            // No need to create error, just move on
            return Ok(CmdResult::Ok);
        }

        let cmd_name = tokens[0];
        let args = &tokens[1..];

        let cmd = self.cmd_map.get(cmd_name).ok_or_else(|| format!("Could not find command named '{}'", cmd_name))?;

        match cmd.execute(args, &mut self.ledger, &mut self.application) {
            Ok(r) => Ok(r),
            Err(CmdError::Syntax(msg)) => {
                // TODO: print usage from cmd object
                eprint!("Syntax Error: ");
                eprintln!("{}", msg);
                // We handled the error, now we can return OK
                Ok(CmdResult::Ok)
            },
            Err(CmdError::Argument(msg)) => {
                Err(Box::new(CmdError::Argument(msg)))
            }
            Err(CmdError::Dependency(err)) => { 
                Err(err) // Pass up dependency errors
            }
        }
    }
}

fn command_list() -> Vec<Rc<dyn Cmd>> {
    vec![
        Rc::new(cmd::account::Account::new()),
        Rc::new(cmd::exit::Exit::new()),
        Rc::new(cmd::load::Load::new()),
        Rc::new(cmd::store::Store::new()),
        Rc::new(cmd::transaction::Transaction::new()),
    ]
}

fn tokenize_string(s: &String) -> Vec<&str> {
    let trimmed = s.trim();
    let split = trimmed.split_whitespace();
    split.collect()
} 

#[cfg(test)]
mod cli_app_tests {

    use std::cell::RefCell;

    use super::*;

    #[test]
    fn create() {
        let _ = CliApp::new(vec![]);
    }

    #[test]
    fn tokenize_empty_string() {
        let s = String::from("");
        assert_eq!(tokenize_string(&s), Vec::<&str>::new())
    }

    #[test]
    fn tokenize_one_token_string() {
        let s = String::from("token");
        assert_eq!(tokenize_string(&s), vec!["token"])
    }

    #[test]
    fn tokenize_multi_token_string() {
        let s = String::from("  this is      a\tstring\n");
        assert_eq!(tokenize_string(&s), vec!["this", "is", "a", "string"])
    }

    struct TestCmd {
        last_called_args: RefCell<Vec<String>>,
        call_count: RefCell<u32>
    }
    impl Cmd for TestCmd {
        fn new() -> Self where Self: Sized {
            TestCmd { last_called_args: RefCell::new(Vec::new()), call_count: RefCell::new(0) }
        }

        fn execute(&self, args: &[&str], _ledger: &mut Ledger, _app: &mut Application) -> Result<CmdResult, CmdError> {
            for arg in args {
                self.last_called_args.borrow_mut().push(String::from(*arg))
            }
            *self.call_count.borrow_mut() += 1;
            Ok(CmdResult::Ok)
        }

        fn names(&self) -> Vec<&'static str> {
            vec!["test", "t"]
        }
    }

    #[test]
    fn test_cmd_dispatch_with_args() {
        let cmd = Rc::new(TestCmd::new());
        let cmds: Vec<Rc<dyn Cmd>> = vec![cmd.clone()];
        let mut app = CliApp::new(cmds);

        assert!(app.run_cmd(&String::from("test arg1 arg2")).is_ok());
        assert_eq!(*cmd.last_called_args.borrow(), vec!["arg1", "arg2"]);
        assert_eq!(*cmd.call_count.borrow(), 1);
    }

    #[test]
    fn test_cmd_dispatch_no_args() {
        let cmd = Rc::new(TestCmd::new());
        let cmds: Vec<Rc<dyn Cmd>> = vec![cmd.clone()];
        let mut app = CliApp::new(cmds);

        assert!(app.run_cmd(&String::from("test")).is_ok());
        assert_eq!(cmd.last_called_args.borrow().len(), 0);
        assert_eq!(*cmd.call_count.borrow(), 1);
    }

    #[test]
    fn test_cmd_invalid_cmd() {
        let cmd = Rc::new(TestCmd::new());
        let cmds: Vec<Rc<dyn Cmd>> = vec![cmd.clone()];
        let mut app = CliApp::new(cmds);

        assert!(app.run_cmd(&String::from("INVALID arg1 arg2")).is_err());
        assert_eq!(*cmd.call_count.borrow(), 0);
    }

}