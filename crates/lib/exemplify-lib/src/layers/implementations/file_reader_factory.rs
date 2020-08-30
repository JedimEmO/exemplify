use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::pin::Pin;
use std::rc::Rc;

use futures::{Stream, StreamExt};
use futures::task::{Context, Poll};

use crate::layers::domain::reader_factory::ReaderFactory;

pub struct FileReaderFactory {
    root_input_directory: Path
}

impl ReaderFactory<File> for FileReaderFactory {
    fn make_reader(&self, name: String) -> Result<File, String> {
        let file_path = Path::new(&name);

        if file_path.is_relative() {
            return Err("Relative file paths are not allowed".into());
        }

        if !file_path.is_file() {
            return Err(format!("{} is not a file", name).into());
        }

        let input_path = self.root_input_directory.join(file_path);

        std::fs::File::open(input_path).map_err(|e| e.to_string())
    }
}
