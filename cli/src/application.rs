//! Represents functionality and data associated with application itself (i.e. not user data). For example, file operations, 
//! os interaction, interaction state, etc. 

use store::FileStore;

pub struct Application {
    pub file_store: FileStore
}

impl Application {
    pub fn new_default() -> Application {
        let exe_path = std::env::current_exe().expect("Failed to get path to exe");
        let exe_dir = exe_path.parent().expect("Could not get exe directory");
        println!("Running in directory: {}", exe_dir.to_str().expect("AH"));
        let file_store = FileStore::new(exe_dir);
        Application { file_store }
    }
}