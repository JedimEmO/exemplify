use std::pin::Pin;

use futures::{Stream, StreamExt};

pub trait Printable {
    fn print(&self) -> String;
    fn file_name(&self) -> String;
}
