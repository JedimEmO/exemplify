use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::pin::Pin;
use std::rc::Rc;

use futures::{Stream, StreamExt};
use futures::task::{Context, Poll};

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
