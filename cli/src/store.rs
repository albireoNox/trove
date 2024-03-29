#[cfg(test)] 
pub mod mock {
    use std::error::Error;
    use ledger::Ledger;

    mockall::mock! {
        pub FileStore { 
            pub fn new(root_path: &std::path::Path) -> Self;
            pub fn store_ledger(&self, ledger: &Ledger) -> Result<(), Box<dyn Error>>;
            pub fn load_ledger(&self) -> Result<Ledger, Box<dyn Error>>;
        }
    }
}
#[cfg(test)]
pub use mock::MockFileStore as FileStore;

#[cfg(not(test))]
pub use store::FileStore;