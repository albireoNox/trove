use std::{fs::File, path::{Path, PathBuf}};

use ciborium::into_writer;
use ledger::Ledger;

static FILE_NAME: &str = "ledger.data";
// For now only version is v0. Once we can commit to stop breaking the format, will change to v1.
static FILE_HEADER: FileHeader = FileHeader { version: 0 };

pub struct FileStore {
    root_path: PathBuf,
}

impl FileStore {
    pub fn new(root_path: &Path) -> FileStore {
        FileStore { root_path: PathBuf::from(root_path) }
    }

    pub fn store_ledger(&self, ledger: &Ledger) -> std::io::Result<()> {
        let file_writer = File::create(self.root_path.join(FILE_NAME))?; 
        let file_data = (&FILE_HEADER, ledger);
        into_writer(&file_data, file_writer).expect("failure");
        Ok(())
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
struct FileHeader {
    version: u32,

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_file_store() {
        let file_store = FileStore::new(Path::new("foo"));
        assert_eq!(file_store.root_path, Path::new("foo"))
    }
}
