use std::{fs::File, path::{Path, PathBuf}, io::{ErrorKind, Error}};

use ciborium::{into_writer, from_reader};
use ledger::Ledger;

static FILE_NAME: &str = "ledger.data";
// For now only version is v0. Once we can commit to stop breaking the format, will change to v1.
static CURRENT_VERSION: u32 = 0;
static FILE_HEADER: FileHeader = FileHeader { version: CURRENT_VERSION };

pub struct FileStore {
    root_path: PathBuf,
}

impl FileStore {
    pub fn new(root_path: &Path) -> FileStore {
        FileStore { root_path: PathBuf::from(root_path) }
    }

    pub fn store_ledger(&self, ledger: &Ledger) -> std::io::Result<()> {
        let file_writer = File::create(self.get_store_file_path())?; 
        into_writer(&FILE_HEADER, &file_writer).expect("failure");
        into_writer(ledger, &file_writer).expect("failure");
        Ok(())
    }

    pub fn load_ledger(&self) -> std::io::Result<Ledger> {
        let file_reader = File::open(self.get_store_file_path())?;
        let file_header: FileHeader = from_reader(&file_reader).expect("failure");
        if file_header.version != CURRENT_VERSION {
            return Err(Error::new(ErrorKind::Unsupported, "Version mismatch, cannot load file"))
        }
        let ledger: Ledger = from_reader(&file_reader).expect("failure");
        Ok(ledger)
    }

    fn get_store_file_path(&self) -> PathBuf {
        self.root_path.join(FILE_NAME)
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
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
