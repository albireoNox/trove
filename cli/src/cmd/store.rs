use ledger::Ledger;

use crate::{cmd::CmdResult, app::Application};

use super::CmdError;

/// Command to save user data
pub struct Store {
}

impl super::Cmd for Store {
    fn new() -> Store {
        Store{}
    }

    fn execute(&self, _args: &[&str], ledger: &mut Ledger, app: &mut Application) -> Result<CmdResult, CmdError> {
        writeln!(app.out(), "Saving user data...")?;
        if let Err(e) = app.store_ledger(ledger) {
            writeln!(app.out(), "Failed to save data!")?;
            return Err(CmdError::Dependency(e))
        }
        writeln!(app.out(), "Saved!")?;
        Ok(CmdResult::Ok)
    }

    fn names(&self) -> Vec<&'static str> {
        vec!["store", "save"]
    }

    fn help_text(&self) -> &'static str {
"Usage: store
Saves data to disk."
    }
}

#[cfg(test)]
mod tests {

    use crate::{cmd::Cmd, store::MockFileStore, ui::MockTerminalInterface};

    use super::*;

    #[test]
    fn execute_store_cmd() { 
        let mut interface = MockTerminalInterface::new();
        let mut file_store = MockFileStore::default();

        let mut test_ledger = Ledger::new_empty();
        test_ledger.add_new_account(String::from("test_account"));

        file_store.expect_store_ledger()
            .times(1)
            .returning(|ledger| {
                assert_eq!(ledger.get_accounts()[0].get_name(), "test_account");
                Ok(())
            });

        interface.expect_write()
            .returning(|s| Ok(s.len()));

        let mut application_mock = Application::new(interface, file_store);

        let store_cmd = Store::new();
        assert!(store_cmd.execute(&vec![], &mut test_ledger, &mut application_mock).is_ok());
    }
}
