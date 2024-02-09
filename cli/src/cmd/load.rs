use ledger::Ledger;

use crate::{cmd::CmdResult, app::Application};

use super::{CmdError, CmdErrorType};

/// Command to load user data from disk
pub struct Load {
}

impl super::Cmd for Load {
    fn new() -> Load {
        Load{}
    }

    fn execute(&self, _args: &[&str], ledger: &mut Ledger, app: &mut Application) -> Result<CmdResult, CmdError> {
        writeln!(app.out(), "Loading user data...")?;
        match app.load_ledger() {
            Ok(new_ledger) => {
                *ledger = new_ledger
            },
            Err(e) => {
                return Err(self.new_error(CmdErrorType::Dependency(e)))
            },
        }
        writeln!(app.out(), "Loaded!")?;
        Ok(CmdResult::Ok)
    }

    fn names(&self) -> Vec<&'static str> {
        vec!["load"]
    }

    fn help_text(&self) -> &'static str {
"Usage: load
Loads saved data from disk."
    }
}

#[cfg(test)]
mod tests {
    use crate::{cmd::Cmd, store::mock::MockFileStore, ui::MockTerminalInterface};

    use super::*;

    #[test]
    fn execute_load_cmd() {
        let mut interface = MockTerminalInterface::new();
        let mut file_store = MockFileStore::default();

        file_store.expect_load_ledger()
            .times(1)
            .returning(|| {
                let mut test_ledger = Ledger::new_empty();
                test_ledger.add_new_account(String::from("test_account"));
                Ok(test_ledger)
            });

        interface.expect_write()
            .returning(|s| Ok(s.len()));

        let mut application_mock = Application::new(interface, file_store);

        let load_cmd = Load::new();
        let mut actual_ledger = Ledger::new_empty();
        assert!(load_cmd.execute(&vec![], &mut actual_ledger, &mut application_mock).is_ok());
        assert!(actual_ledger.get_account_by_name_mut(&String::from("test_account")).is_some());
    }
}
