use std::io::{stdin, stdout, Stdout, Write};

use colored::Colorize;
use termion::{event::Key, input::TermRead, raw::{IntoRawMode, RawTerminal}};

#[derive(Clone)]
pub enum InputEvent {
    Text(String), 
    ArrowUp, 
    ArrowDown,
    Interrupt,
}

#[allow(dead_code)] // needed due to mock messing things up
pub struct TerminalInterface {
    input_buffer: String,
    _terminal: RawTerminal<Stdout>, // This is saved so the original terminal state can be restored on drop. 
}

#[allow(dead_code)] // needed due to mock messing things up
impl TerminalInterface {
    pub fn create() -> std::io::Result<Self> {
        let raw_terminal = stdout().into_raw_mode()?;

        Ok(TerminalInterface {
            input_buffer: String::new(),
            _terminal: raw_terminal,
        })
    }

    /// Waits for and returns the next user input event. Will block until enough keys are pressed to need to do something. 
    pub fn get_event(&mut self) -> InputEvent {

        let mut keys = stdin().keys();

        // listen for key events
        loop {
            self.display_input_buffer();

            let next_key = keys.next();

            if let Some(key) = next_key {
                match key.unwrap() {
                    Key::Char('\n') => {
                        let text = self.input_buffer.trim().to_string();
                        let event = InputEvent::Text(text);
                        self.input_buffer.clear();
                        print!("\n\r");
                        return event;
                    },
                    Key::Char(ch) => {
                        self.input_buffer.push(ch);
                    },
                    Key::Backspace => {
                        self.input_buffer.pop();
                    },
                    Key::Up => {
                        return InputEvent::ArrowUp;
                    }, 
                    Key::Down => {
                        return InputEvent::ArrowDown;
                    }
                    Key::Ctrl('c') => {
                        return InputEvent::Interrupt;
                    },
                    _ => {}
                }
            } else { // we've reached the end of the keys, probably because we're shutting down
                return InputEvent::Interrupt;
            }
        }
    }

    fn display_input_buffer(&self) {
        print!("{}\r{}{}", 
            termion::clear::CurrentLine,
            "> ".green(), 
            self.input_buffer);
        stdout().flush().unwrap();
    }

    pub fn set_input_buffer(&mut self, s: String) {
        self.input_buffer = s;

        self.display_input_buffer();
    }
}

impl std::io::Write for TerminalInterface {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let s = String::from_utf8_lossy(buf);
        stdout().write_all(s.replace('\n', "\n\r").as_bytes())?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        stdout().flush()
    }
}

#[cfg(test)] 
mockall::mock! {
    pub TerminalInterface {
        pub fn create() -> std::io::Result<Self>;
        pub fn get_event(&mut self) -> InputEvent;
        pub fn set_input_buffer(&mut self, s: String);
    }
    impl std::io::Write for TerminalInterface { 
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize>;
        fn flush(&mut self) -> std::io::Result<()>;
    }
}