//! This crate builds into an executible for running the CLI application. 
 
// In general, code that could apply to different types of applications (GUI. 
// web, etc.) should go elsewhere. 

use std::{io::{self, Write}, error::Error, collections::HashMap, rc::Rc};
use colored::*;
use application::Application;
use cmd::{Cmd, CmdError, CmdResult};
use ledger::Ledger;

mod cmd;
mod application;

static ESCAPE_CHAR: char = '\\';
static QUOTE_CHARS: [char; 2] = ['\'', '"'];

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
    cmd_list: Vec<Rc<dyn Cmd>>,
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
            cmd_list: cmds,
            ledger: Ledger::new_empty(), // TODO: load exiting one
            application: application
        }
    }

    // This function is not easily testible, since it reads from stdin. TODO: consider refactoring this out somehow. 
    fn run(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            Self::print_prompt()?;

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
        let tokens_owned = tokenize_string(raw_input);
        let tokens: Vec<&str> = tokens_owned.iter().map(|s| s.as_str()).collect();

        if tokens.len() == 0 {
            // No need to create error, just move on
            return Ok(CmdResult::Ok);
        }

        let cmd_name = &tokens[0];
        let args = &tokens[1..];

        if cmd_name.eq_ignore_ascii_case("help") {
            self.print_help(args);
            return Ok(CmdResult::Ok)
        }

        let cmd = self.cmd_map.get(cmd_name).ok_or_else(|| format!("Could not find command named '{}'", cmd_name))?;

        if args.get(0).is_some_and(|arg| arg.eq_ignore_ascii_case("--help")) {
            println!("{}", cmd.help_text());
            return Ok(CmdResult::Ok)
        }

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

    fn print_prompt() -> Result<(), Box<dyn Error>> {
        print!("{}", "> ".green());
        io::stdout().flush()?;
        Ok(())
    }

    fn print_help(&self, args: &[&str]) {
        match args.get(0) {
            Some(cmd_name) => { 
                let cmd = self.cmd_map.get(cmd_name);
                match cmd {
                    Some(c) => {
                        println!("{}", c.help_text());
                    }
                    None => {
                        println!("No command named '{}'", cmd_name);
                    }
                }
            },
            None => { 
                println!("The following commands are available:\n");
                for cmd in &self.cmd_list {
                    print!("  {}", cmd.names()[0]);
                    let aliases = &cmd.names()[1..];
                    if aliases.len() > 0 {
                        println!("  ({})", aliases.join(", "))
                    } else {
                        println!();
                    }
                }
                println!("\n'help COMMAND' will list detailed information on a given command.");
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

fn tokenize_string(s: &String) -> Vec<String> {
    let mut tokens = Vec::<String>::new();

    let mut escaped = false;
    let mut cur_token: Option<String> = None;
    let mut opened_quote: Option<char> = None;

    fn add_char_to_token(ch: char, token: &mut Option<String>) {
        match token.as_mut() {
            Some(token_string) => token_string.push(ch),
            None => *token = Some(String::from(ch)),
        }
    }

    fn finalize_token(token: Option<String>, tokens: &mut Vec<String>) -> Option<String> {
        if let Some(token_string) = token {
            tokens.push(token_string)
        }
        None
    }

    for ch in s.chars() {
        if escaped { // Escaped means we just add the character no matter what it is
            add_char_to_token(ch, &mut cur_token);
            escaped = false;
        } else if let Some(quote_char) = opened_quote { // If we're in a quoted part...
            if ch == quote_char {                       // and this is the end of the quote...
                opened_quote = None;                    // then mark the quoted part as being over;...
            } else {                                    // but if we're still inside the quoted part...
                add_char_to_token(ch, &mut cur_token)   // just add the character. 
            }
        } else if ch == ESCAPE_CHAR { // Escape the NEXT character. 
            escaped = true;
        } else if QUOTE_CHARS.contains(&ch) { // Begin quoted part. 
            opened_quote = Some(ch);
        } else if ch.is_whitespace() {
            cur_token = finalize_token(cur_token, &mut tokens);
        } else { 
            add_char_to_token(ch, &mut cur_token)
        }
    }

    // Add whatever token we were working on when the string ended. 
    finalize_token(cur_token, &mut tokens);

    tokens
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
    fn tokenize_string_with_only_whitespace() {
        let s = String::from("  \t \n");
        assert_eq!(tokenize_string(&s), Vec::<&str>::new())
    }
    
    #[test]
    fn tokenize_string_with_only_escaped_whitespace() {
        let s = String::from(" \\   ");
        assert_eq!(tokenize_string(&s), vec![" "])
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

    #[test]
    fn tokenize_with_quote() {
        let s = String::from("this is \"a string\" ");
        assert_eq!(tokenize_string(&s), vec!["this", "is", "a string"])
    }

    #[test]
    fn tokenize_with_partially_quoted_token() {
        let s = String::from("\"this is\"a string");
        assert_eq!(tokenize_string(&s), vec!["this isa", "string"])
    }

    #[test]
    fn tokenize_with_two_quoted_token_parts() {
        let s = String::from("\"this is\"' a string'");
        assert_eq!(tokenize_string(&s), vec!["this is a string"])
    }

    #[test]
    fn tokenize_with_single_quote() {
        let s = String::from("this is 'a string'  ");
        assert_eq!(tokenize_string(&s), vec!["this", "is", "a string"])
    }

    #[test]
    fn tokenize_with_escape() {
        let s = String::from(r"this is a\ string  ");
        assert_eq!(tokenize_string(&s), vec!["this", "is", "a string"])
    }

    #[test]
    fn tokenize_with_escape_inside_quote() {
        let s = String::from("this is \"a\\ string\" ");
        assert_eq!(tokenize_string(&s), vec!["this", "is", "a\\ string"])
    }

    #[test]
    fn tokenize_with_interior_quote_mark() {
        let s = String::from("\"tok'en\"");
        assert_eq!(tokenize_string(&s), vec!["tok'en"])
    }

    #[test]
    fn tokenize_with_double_escape() {
        let s = String::from(r"tok\\en");
        assert_eq!(tokenize_string(&s), vec![r"tok\en"])
    }

    #[test]
    fn tokenize_with_escaped_normal_character() {
        let s = String::from(r"tok\en");
        assert_eq!(tokenize_string(&s), vec![r"token"])
    }

    #[test]
    fn tokenize_with_escaped_quotes() {
        let s = String::from("\\\"token\\\"");
        assert_eq!(tokenize_string(&s), vec!["\"token\""])
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

        fn help_text(&self) -> &'static str {
            "TEST"
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