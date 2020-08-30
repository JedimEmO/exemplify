/// This is a raw example chunk - what we extract from the individual source files
#[derive(Default, Clone)]
pub struct Chunk {
    pub example_name: String,
    pub content: Vec<String>,
    pub part_number: Option<u32>,
}
