//! This crate builds into an executible for running the CLI application. 
 
// In general, code that could apply to different types of applications (GUI. 
// web, etc.) should go elsewhere. 

use std::{io, error::Error, collections::HashMap, cell::RefCell, rc::Rc};
use application::Application;
use cmd::{Cmd, exit::Exit};

mod cmd;
mod application;

fn main() -> Result<(), Box<dyn Error>> {
    let app = CliApp::new();
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
    terminated: RefCell<bool>
}

impl CliApp {
    fn new() -> CliApp {
        let mut app = CliApp {
            cmds: Vec::new(),
            cmd_map: HashMap::new(),
            terminated: RefCell::new(false)
        };
    
        app.cmds.push(Rc::new(Exit::new()));

        for cmd in &app.cmds {
            for name in cmd.names() {
                app.cmd_map.insert(name, cmd.clone());
            }
        }

        app
    }

    fn run(&self) -> Result<(), Box<dyn Error>> {
        while !(*self.terminated.borrow()) {
            if let Err(e) = self.run_one_command() {
                // For now all errors are recoverable
                eprintln!("{}", e);
            }
        }

        Ok(())
    }

    fn run_one_command(&self) -> Result<(), Box<dyn Error>> {
        let mut raw_input = String::new();
    
        io::stdin().read_line(&mut raw_input)?;
    
        let trimmed_input = raw_input.trim();
    
        if trimmed_input.len() == 0 {
            // No need to create error, just move on
            return Ok(());
        }
    
        let mut split = trimmed_input.split_whitespace();
        let cmd_name = split.next().unwrap_or("");
        let args: Vec<&str> = split.collect();
    
        let cmd = self.cmd_map.get(cmd_name).ok_or_else(|| format!("Could not find command named '{}'", cmd_name))?;

        cmd.execute(args, self)
    }
}

impl Application for CliApp {
    fn signal_terminate(&self) {
        *self.terminated.borrow_mut() = true;
    }
}