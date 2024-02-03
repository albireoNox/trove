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

    use crate::cmd::Cmd;

    use super::*;

    static mut TEST_OUTPUT: Vec<u8> = Vec::new();

    #[test]
    fn execute_store_cmd() { 
        let mut application_mock = Application::faux();

        let mut test_ledger = Ledger::new_empty();
        test_ledger.add_new_account(String::from("test_account"));
        
        let mut stored = false;
        unsafe { 
            faux::when!(
                application_mock.store_ledger(_)
            ).then_unchecked(|ledger| {
                assert_eq!(ledger as *const Ledger, &test_ledger as *const Ledger); // reference equality
                stored = true;
                Ok(())
            });

            faux::when!(
                application_mock.out(_)
            ).then_unchecked(|_| &mut TEST_OUTPUT); 
        }

        let store_cmd = Store::new();

        assert!(store_cmd.execute(&vec![], &mut test_ledger, &mut application_mock).is_ok());
        assert!(stored);
        unsafe { assert!(TEST_OUTPUT.len() > 0); }
    }
}
