use ledger::Ledger;

use crate::{cmd::CmdResult, application::Application};

use super::CmdError;

/// Command to save user data
pub struct Store {
}

impl super::Cmd for Store {
    fn new() -> Store {
        Store{}
    }

    fn execute(&self, _args: &[&str], ledger: &mut Ledger, app: &mut Application) -> Result<CmdResult, CmdError> {
        println!("Saving user data...");
        if let Err(e) = app.store_ledger(ledger) {
            eprintln!("Failed to save data!");
            return Err(CmdError::Dependency(e))
        }
        println!("Saved!");
        Ok(CmdResult::Ok)
    }

    fn names(&self) -> Vec<&'static str> {
        vec!["store", "save"]
    }
}

#[cfg(test)]
mod tests {

    use crate::cmd::Cmd;

    use super::*;

    #[test]
    fn execute_store_cmd() {
        let mut application_mock = Application::faux();

        let mut test_ledger = Ledger::new_empty();
        test_ledger.add_new_account(String::from("test_account"));
        
        let mut stored = false;
        unsafe { faux::when!(
            application_mock.store_ledger(_)
        ).then_unchecked(|ledger| {
            assert_eq!(ledger as *const Ledger, &test_ledger as *const Ledger); // reference equality
            stored = true;
            Ok(())
        });}

        let store_cmd = Store::new();

        assert!(store_cmd.execute(&vec![], &mut test_ledger, &mut application_mock).is_ok());
        assert!(stored);
    }
}
