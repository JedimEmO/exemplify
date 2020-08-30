use std::fs::File;

use std::path::Path;






use crate::layers::domain::reader_factory::ReaderFactory;

pub struct FileReaderFactory {}

impl ReaderFactory<File> for FileReaderFactory {
    fn make_reader(&self, name: String) -> Result<File, String> {
        let file_path = Path::new(&name);

        if !file_path.is_file() {
            return Err(format!("{} is not a file", name).into());
        }

        std::fs::File::open(file_path).map_err(|e| e.to_string())
    }
}
