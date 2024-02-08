//! Overall container for data associated with the application itself (i.e. not user data). For example, file operations, 
//! os interaction, etc. 

/// Ideally this is just a bundle of owned structs which serve to actually manage the concerns listed above. 
/// For the sake of testing, this should not perform any "untestible" os operations directly. Rather, 
/// the sub-structs can be mocked as needed and passed in. 

use std::error::Error;

use ledger::Ledger;

#[cfg(test)]
pub use crate::test::store;
#[mockall_double::double]
use store::FileStore;

#[mockall_double::double]
use crate::ui::TerminalInterface;

pub struct Application {
    file_store: FileStore,
    interface: TerminalInterface,
}

impl Application {
    pub fn new(interface: TerminalInterface, file_store: FileStore) -> Self {
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