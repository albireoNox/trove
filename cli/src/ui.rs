use std::io::{stdin, stdout, Stdout, Write};

use colored::Colorize;
use termion::{event::Key, input::TermRead, raw::{IntoRawMode, RawTerminal}};

pub struct TerminalInterface {
    input_buffer: String,
    _terminal: RawTerminal<Stdout>, // This is saved so the original terminal state can be restored on drop. 
}

pub enum InputEvent {
    Text(String), 
    Terminate,
}

impl TerminalInterface {
    pub fn create() -> std::io::Result<TerminalInterface> {
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

            // update the current line with the state of the buffer
            print!("{}\r{}{}", 
                termion::clear::CurrentLine,
                "> ".green(), 
                self.input_buffer);
            stdout().flush().unwrap();

            let next_key = keys.next();

            if let Some(key) = next_key {
                match key.unwrap() {
                    Key::Char('\n') => {
                        let event = InputEvent::Text(self.input_buffer.clone());
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
                    Key::Ctrl('c') => {
                        return InputEvent::Terminate;
                    },
                    _ => {}
                }
            } else { // we've reached the end of the keys, probably because we're shutting down
                return InputEvent::Terminate;
            }
        }
    }
}

impl Write for TerminalInterface {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let s = String::from_utf8_lossy(buf);
        stdout().write(s.replace("\n", "\n\r").as_bytes())?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        stdout().flush()
    }
}