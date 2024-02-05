//! This crate builds into an executible for running the CLI application. 
 
// In general, code that could apply to different types of applications (GUI. 
// web, etc.) should go elsewhere. 

use std::{collections::{HashMap, VecDeque}, error::Error, rc::Rc};
use app::Application;
use cmd::{Cmd, CmdError, CmdResult};
use ledger::Ledger;
use ui::TerminalInterface;

mod app;
mod cmd;
mod ui;

const ESCAPE_CHAR: char = '\\';
const QUOTE_CHARS: [char; 2] = ['\'', '"'];
const MAX_HISTORY_LENGTH: usize = 100;

fn main() -> Result<(), Box<dyn Error>> {
    let interface = TerminalInterface::create()?;
    let application = Application::new_default(interface);
    let mut cli_app = CliRunner::create(command_list(), application)?;

    if let Err(e) = cli_app.run() {
        eprintln!("Encountered fatal error: {e}");
        eprintln!("Exiting...");

        return Err(e);
    }

    Ok(())
}

/// State used by the CLI application. Manages the top-level REPL loop and parses input
/// to dispatch to command structs. 
struct CliRunner {
    cmd_map: HashMap<&'static str, Rc<dyn Cmd>>,
    cmd_list: Vec<Rc<dyn Cmd>>,
    ledger: Ledger,
    app: Application,

    // Most recent history entry is at index 0. This might be more efficient as a linked-list, 
    // but maybe not. The gains from not having to shift elements may be less than what we get
    // from the memory locality of an array-based list. 
    input_history: VecDeque<String>, 
}

impl CliRunner {
    fn create(cmds: Vec<Rc<dyn Cmd>>, app: Application) -> Result<CliRunner, Box<dyn Error>> {
        let mut cmd_map = HashMap::new();
        for cmd in &cmds {
            for name in cmd.names() {
                cmd_map.insert(name, cmd.clone());
            }
        }

        Ok(CliRunner {
            cmd_map,
            cmd_list: cmds,
            ledger: Ledger::new_empty(), // TODO: load exiting one
            app,
            input_history: VecDeque::new(),
        })
    }

    // TODO: Write tests for this function
    fn run(&mut self) -> Result<(), Box<dyn Error>> {

        loop {
            let event = self.app.interface().get_event();
            
            match self.handle_input_event(event) {
                Ok(false) => break,
                Ok(true) => continue,
                Err(e) => {
                    // For now all errors are recoverable
                    writeln!(self.app.out(), "{}", e)?; 
                }
            }
        }

        Ok(())
    }

    /// Return Ok(true) if we should keep listing for events, and Ok(false) if we should terminate
    fn handle_input_event(&mut self, event: ui::InputEvent) -> Result<bool, Box<dyn Error>> {
        match event {
            ui::InputEvent::Text(s) => {
                let result = match self.run_cmd(&s) { 
                    Ok(CmdResult::SignalTerminate) => Ok(false),
                    Ok(CmdResult::Ok) => Ok(true), 
                    Err(e) => Err(e)
                };
                self.add_input_to_history(s);
                result
            },
            ui::InputEvent::ArrowUp => self.page_history(),
            ui::InputEvent::ArrowDown => self.page_history(),
            ui::InputEvent::Interrupt => Ok(false),
        }
    }

    fn page_history(&mut self) -> Result<bool, Box<dyn Error>> {
        // If there's no history then there's nothing to do here. 
        if self.input_history.is_empty() {
            return Ok(true)
        }

        // Cycle through the history until we get a different kind of event. 
        let mut history_index = 0;
        loop {
            self.app.interface().set_input_buffer(self.input_history[history_index].clone());

            let next_event = self.app.interface().get_event();
            match next_event {
                ui::InputEvent::ArrowUp => {
                    history_index = (history_index + 1) % self.input_history.len();
                },
                ui::InputEvent::ArrowDown => {
                    history_index = (history_index + self.input_history.len() - 1) % self.input_history.len();
                },
                _ => return self.handle_input_event(next_event)
            }
        }
    }

    fn add_input_to_history(&mut self, input: String) {
        // Dedup the history entries
        self.input_history.retain(|s| !s.eq_ignore_ascii_case(&input));
        self.input_history.push_front(input);

        if self.input_history.len() > MAX_HISTORY_LENGTH {
            self.input_history.pop_back(); // Remove oldest entry
        }
    }

    fn run_cmd(&mut self, raw_input: &str) -> Result<CmdResult, Box<dyn Error>> {
        let tokens_owned = tokenize_string(raw_input);
        let tokens: Vec<&str> = tokens_owned.iter().map(|s| s.as_str()).collect();

        if tokens.is_empty() {
            // No need to create error, just move on
            return Ok(CmdResult::Ok);
        }

        let cmd_name = &tokens[0];
        let args = &tokens[1..];

        if cmd_name.eq_ignore_ascii_case("help") {
            self.print_help(args)?;
            return Ok(CmdResult::Ok)
        }

        let cmd = self.cmd_map.get(cmd_name).ok_or_else(|| format!("Could not find command named '{}'", cmd_name))?;

        if args.first().is_some_and(|arg| arg.eq_ignore_ascii_case("--help")) {
            writeln!(self.app.out(), "{}", cmd.help_text())?;
            return Ok(CmdResult::Ok)
        }

        match cmd.execute(args, &mut self.ledger, &mut self.app) {
            Ok(r) => Ok(r),
            Err(CmdError::Syntax(msg)) => {
                // TODO: print usage from cmd object
                write!(self.app.out(), "Syntax Error: ")?;
                writeln!(self.app.out(), "{}", msg)?;
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

    fn print_help(&mut self, args: &[&str]) -> Result<(), Box<dyn Error>> {
        match args.first() {
            Some(cmd_name) => { 
                let cmd = self.cmd_map.get(cmd_name);
                match cmd {
                    Some(c) => {
                        writeln!(self.app.out(), "{}", c.help_text())?;
                    }
                    None => {
                        writeln!(self.app.out(), "No command named '{}'", cmd_name)?;
                    }
                }
            },
            None => { 
                writeln!(self.app.out(), "The following commands are available:\n")?;
                for cmd in &self.cmd_list {
                    write!(self.app.out(), "  {}", cmd.names()[0])?;
                    let aliases = &cmd.names()[1..];
                    if !aliases.is_empty() {
                        writeln!(self.app.out(), "  ({})", aliases.join(", "))?;
                    } else {
                        writeln!(self.app.out())?;
                    }
                }
                writeln!(self.app.out(), "\n'help COMMAND' will list detailed information on a given command.")?;
            }
        }

        Ok(())
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

fn tokenize_string(s: &str) -> Vec<String> {
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

    use std::{cell::{OnceCell, RefCell}, sync::{Arc, Mutex}};

    use super::*;

    #[test]
    fn create() {
        let _ = CliRunner::create(vec![], Application::faux());
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
        let mut runner = CliRunner::create(cmds, Application::faux()).unwrap();

        assert!(runner.run_cmd(&String::from("test arg1 arg2")).is_ok());
        assert_eq!(*cmd.last_called_args.borrow(), vec!["arg1", "arg2"]);
        assert_eq!(*cmd.call_count.borrow(), 1);
    }

    #[test]
    fn test_cmd_dispatch_no_args() {
        let cmd = Rc::new(TestCmd::new());
        let cmds: Vec<Rc<dyn Cmd>> = vec![cmd.clone()];
        let mut runner = CliRunner::create(cmds, Application::faux()).unwrap();

        assert!(runner.run_cmd(&String::from("test")).is_ok());
        assert_eq!(cmd.last_called_args.borrow().len(), 0);
        assert_eq!(*cmd.call_count.borrow(), 1);
    }

    #[test]
    fn test_cmd_invalid_cmd() {
        let cmd = Rc::new(TestCmd::new());
        let cmds: Vec<Rc<dyn Cmd>> = vec![cmd.clone()];
        let mut runner = CliRunner::create(cmds, Application::faux()).unwrap();

        assert!(runner.run_cmd(&String::from("INVALID arg1 arg2")).is_err());
        assert_eq!(*cmd.call_count.borrow(), 0);
    }

    #[test]
    fn test_add_input_to_history_dedup() {
        let mut runner = CliRunner::create(vec![], Application::faux()).unwrap();

        runner.add_input_to_history("cmd1".to_string());
        runner.add_input_to_history("cmd2".to_string());
        runner.add_input_to_history("cmd3".to_string());
        runner.add_input_to_history("cmd2".to_string());

        assert_eq!(runner.input_history, vec!["cmd2", "cmd3", "cmd1"])
    }

    #[test]
    fn test_add_input_to_history_over_max() {
        let mut runner = CliRunner::create(vec![], Application::faux()).unwrap();

        for i in 0..=MAX_HISTORY_LENGTH+1 {
            runner.add_input_to_history(format!("cmd{}", i));
        }

        assert_eq!(runner.input_history.len(), MAX_HISTORY_LENGTH);
        assert_eq!(runner.input_history.front().unwrap(), "cmd101");
        assert_eq!(runner.input_history.back().unwrap(), "cmd2");
    }

    // This test is bananas. TODO: Find a mocking library that doesn't require this madness. Or refactor things. 
    #[test]
    fn test_command_history() {
        unsafe {

        let mut events = VecDeque::from(vec![
            ui::InputEvent::Text("cmd1".to_string()),
            ui::InputEvent::Text("cmd2".to_string()),
            ui::InputEvent::Text("cmd3".to_string()),
            ui::InputEvent::ArrowUp,
            ui::InputEvent::ArrowUp,
            ui::InputEvent::ArrowUp,
            ui::InputEvent::ArrowDown,
            ui::InputEvent::Interrupt,
        ]);

        let expected_cmd_history = Arc::new(Mutex::new(VecDeque::from(vec![
            "cmd3", "cmd2", "cmd1", "cmd2"
        ])));

        let mut app = Application::faux();
        let mut interface = TerminalInterface::faux();

        let copy = expected_cmd_history.clone();
        faux::when!(interface.set_input_buffer).then(move |s| 
            assert_eq!(s, copy.lock().unwrap().pop_front().unwrap()));

        faux::when!(interface.get_event()).then_unchecked(|_| events.pop_front().unwrap());

        static mut CELL: OnceCell<TerminalInterface> = OnceCell::new();
        CELL.set(interface).unwrap();
        faux::when!(app.interface()).then_unchecked(|_| CELL.get_mut().unwrap());

        let test_out = Vec::<u8>::new();
        static mut OUT_CELL: OnceCell<Vec<u8>> = OnceCell::new();
        OUT_CELL.set(test_out).unwrap();
        faux::when!(app.out).then_unchecked(|_| OUT_CELL.get_mut().unwrap());

        CliRunner::create(Vec::new(), app).unwrap().run().unwrap();
        assert!(expected_cmd_history.lock().unwrap().is_empty())
        }
    }

    // This needs to be here for silly reasons. 
    impl std::fmt::Debug for TerminalInterface {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "this doesn't matter")
        }
    }
}