use std::io::{BufRead, BufReader, Read};
use std::pin::Pin;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

use futures::stream::Stream;
use futures::task::{Context, Poll};

use crate::layers::domain::chunk::Chunk;
use crate::layers::domain::parser_settings::ParserSettings;

pub struct ChunkReader<Reader> {
    reader: Arc<Mutex<BufReader<Reader>>>,
    parser_settings: ParserSettings,
    current_chunk: Option<Chunk>,
}

impl<Reader: Read> Stream for ChunkReader<Reader> {
    type Item = Result<Vec<Chunk>, String>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut completed_chunks = Vec::new();

        let rc = self.reader.clone();
        let mut reader = rc.lock().map_err(|e| { e.to_string() })?;

        let taken_lines = reader.by_ref().lines().take(100);
        let mut read_count = 0;

        for line in taken_lines {
            read_count += 1;

            match line {
                Err(err) => return Poll::Ready(Some(Err(err.to_string()))),
                Ok(line) => {
                    if let Some(result) = self.process_line(&line)? {
                        completed_chunks.push(result);
                    }
                }
            }
        }

        if completed_chunks.len() > 0 {
            Poll::Ready(Some(Ok(completed_chunks)))
        } else if read_count == 0 {
            Poll::Ready(None)
        } else {
            cx.waker().clone().wake();

            Poll::Pending
        }
    }
}

impl<Reader: Read> ChunkReader<Reader> {
    pub fn new(reader: Reader, parser_settings: ParserSettings) -> Self {
        Self {
            reader: Arc::new(Mutex::new(BufReader::new(reader))),
            parser_settings,
            current_chunk: None,
        }
    }

    fn process_line(self: &mut Pin<&mut Self>, line: &String) -> Result<Option<Chunk>, String> {
        let has_start = line.contains(&self.parser_settings.start_token);
        let has_end = line.contains(&self.parser_settings.end_token);

        match &mut self.current_chunk {
            Some(chunk) => {
                if has_start {
                    return Err("Error: attempting to start chunk-in-chunk".into());
                }

                if has_end {
                    return self.finalize_chunk();
                }

                if !has_start && !has_end {
                    chunk.content.push(line.clone());
                }

                Ok(None)
            }
            None => {
                if has_start {
                    let params = Self::extract_chunk_params(&line)?;
                    self.current_chunk = Some(Chunk {
                        example_name: params.name,
                        content: vec![],
                        part_number: params.part,
                    });
                    return Ok(None);
                }

                if has_end {
                    return Err("Error: attempting to end chunk outside of chunk".into());
                }

                Ok(None)
            }
        }
    }

    fn extract_chunk_params(line: &String) -> Result<ChunkParams, String> {
        lazy_static::lazy_static! {
            static ref VAL_RE: regex::Regex = regex::Regex::new("\\{(([a-zA-Z]+)\\s?=\\s?\"([a-zA-Z\\s0-9\\-/]+)\")|(([a-zA-Z]+)\\s?=\\s?([0-9]+))\\}").unwrap();        }

        let mut name: String = "".into();
        let mut part = None;

        for val in VAL_RE.captures_iter(line) {
            let param_name_name = val.get(2);
            let param_name_val = val.get(3);

            let param_part_name = val.get(5);
            let param_part_val = val.get(6);


            if let Some(_) = param_name_name {
                if let Some(n) = param_name_val {
                    name = n.as_str().to_string().clone();
                }
            }

            if let Some(_) = param_part_name {
                if let Some(part_val) = param_part_val {
                    part = Some(u32::from_str(part_val.as_str())
                        .map_err(|_| format!("Failed to parse part number {}", part_val.as_str().to_string()))?);
                }
            }
        }

        if name.len() == 0 {
            return Err("Missing name".into());
        }

        println!("name: {}", name);

        Ok(ChunkParams {
            part,
            name,
        })
    }

    fn finalize_chunk(self: &mut Pin<&mut Self>) -> Result<Option<Chunk>, String> {
        Ok(self.current_chunk.take())
    }
}

struct ChunkParams {
    name: String,
    part: Option<u32>,
}
