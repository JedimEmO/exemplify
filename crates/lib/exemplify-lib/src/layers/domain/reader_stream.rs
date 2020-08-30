use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::pin::Pin;
use std::rc::Rc;

use futures::{Stream, StreamExt};
use futures::task::{Context, Poll};

use crate::layers::domain::reader_factory::ReaderFactory;

pub fn reader_stream<Reader: Read + 'static>(
    reader_factory: Box<dyn ReaderFactory<Reader>>,
    file_path_stream: Pin<Box<dyn Stream<Item=Result<String, String>>>>) -> Pin<Box<dyn Stream<Item=Result<Reader, String>>>> {
    Box::pin(file_path_stream.map(move |path| {
        match path {
            Err(e) => Err(e),
            Ok(path) => reader_factory.make_reader(path)
        }
    }))
}
