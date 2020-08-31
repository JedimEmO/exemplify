use std::io::Read;

pub trait ReaderFactory<Reader: Read> {
    fn make_reader(&self, name: String) -> Result<ReaderContext<Reader>, String>;
}

pub struct ReaderContext<Reader: Read> {
    pub reader: Reader,
    pub source_name: String
}
