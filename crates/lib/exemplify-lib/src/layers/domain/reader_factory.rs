use std::io::Read;

pub trait ReaderFactory<Reader: Read> {
    fn make_reader(&self, name: String) -> Reader;
}
