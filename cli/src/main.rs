//! This crate builds into an executible for running the CLI application. 
 
// In general, code that could apply to different types of applications (GUI. 
// web, etc.) should go elsewhere. 

use std::{io, error::Error, collections::HashMap, rc::Rc};
use cmd::{Cmd, CmdError, CmdResult};
use ledger::Ledger;

mod cmd;

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = CliApp::new();
    if let Err(e) = app.run() {
        eprintln!("Encountered fatal error: {e}");
        eprintln!("Exiting...");

        return Err(e);
    }

    Ok(())
}

/// State used by the CLI application. Manages the top-level REPL loop and parses input
/// to dispatch to command structs. 
struct CliApp {
    cmds: Vec<Rc<dyn Cmd>>,
    cmd_map: HashMap<&'static str, Rc<dyn Cmd>>,
    ledger: Ledger,
}

impl CliApp {
    fn new() -> CliApp {
        let mut app = CliApp {
            cmds: Vec::new(),
            cmd_map: HashMap::new(),
            ledger: Ledger::new_empty(), // TODO: load exiting one
        };

        app.register_cmds();

        app
    }

    fn run(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            match self.run_one_command() {
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

    fn run_one_command(&mut self) -> Result<CmdResult, Box<dyn Error>> {
        let mut raw_input = String::new();
    
        io::stdin().read_line(&mut raw_input)?;
    
        let trimmed_input = raw_input.trim();
    
        if trimmed_input.len() == 0 {
            // No need to create error, just move on
            return Ok(CmdResult::Ok);
        }
    
        let mut split = trimmed_input.split_whitespace();
        let cmd_name = split.next().unwrap_or("");
        let args: Vec<&str> = split.collect();
    
        let cmd = self.cmd_map.get(cmd_name).ok_or_else(|| format!("Could not find command named '{}'", cmd_name))?;

        match cmd.execute(args, &mut self.ledger) {
            Ok(r) => Ok(r),
            Err(CmdError::Syntax(msg)) => {
                // TODO: print usage from cmd object
                eprint!("Syntax Error: ");
                eprintln!("{}", msg);
                // We handled the error, now we can return OK
                Ok(CmdResult::Ok)
            },
        }
    }

    fn register_cmds(&mut self) {
        self.cmds.push(Rc::new(cmd::exit::Exit::new()));
        self.cmds.push(Rc::new(cmd::account::Account::new()));

        for cmd in &self.cmds {
            for name in cmd.names() {
                self.cmd_map.insert(name, cmd.clone());
            }
        }
    }
}