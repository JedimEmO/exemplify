/// This is a raw example chunk - what we extract from the individual source files
#[derive(Default, Clone)]
pub struct Chunk {
    pub example_name: String,
    pub content: Vec<ChunkLine>,
    pub part_number: Option<u32>,
    pub indentation: Option<u32>,
    pub source_name: String,
    pub start_line: usize
}

#[derive(Default, Clone)]
pub struct ChunkLine {
    pub value: String,
    pub line_number: usize
}
