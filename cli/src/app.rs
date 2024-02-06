//! Represents functionality and data associated with application itself (i.e. not user data). For example, file operations, 
//! os interaction, interaction state, etc. 

use std::error::Error;

use ledger::Ledger;
use store::FileStore;

use crate::ui::TerminalInterface;

#[cfg_attr(test, faux::create)]
pub struct Application {
    file_store: FileStore,
    interface: TerminalInterface,
}

#[cfg_attr(test, faux::methods)]
impl Application {
    pub fn new(interface: TerminalInterface, file_store: FileStore) -> Application {
        Application { file_store, interface }
    }

    pub fn store_ledger(&self, ledger: &Ledger) -> Result<(), Box<dyn Error>> {
        self.file_store.store_ledger(ledger)
    }

    pub fn load_ledger(&self) -> Result<Ledger, Box<dyn Error>> {
        self.file_store.load_ledger()   
    }

    pub fn out(&mut self) -> &mut dyn std::io::Write {
        &mut self.interface
    }

    pub fn interface(&mut self) -> &mut TerminalInterface {
        &mut self.interface
    }
}