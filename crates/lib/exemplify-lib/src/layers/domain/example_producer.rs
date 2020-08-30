use std::collections::{HashMap, HashSet};
use std::io::Read;

use futures::{StreamExt};

use crate::layers::domain::chunk::Chunk;
use crate::layers::domain::chunk_reader::ChunkReader;
use crate::layers::domain::parser_settings::ParserSettings;
use crate::layers::domain::reader_factory::ReaderFactory;
use std::cmp::Ordering;

pub struct Example {
    pub name: String,
    content: Vec<String>,
}

impl Example {
    fn new(name: String, content: Vec<String>) -> Example {
        Example {
            name,
            content,
        }
    }
}

pub struct ExampleService<Reader: Read> {
    reader_factory: Box<dyn ReaderFactory<Reader>>,
    reader_queue: Vec<String>,
    active_reader: Option<ChunkReader<Reader>>,
    parser_settings: ParserSettings,
    chunk_cache: HashMap<String, Vec<Chunk>>,
}


impl<Reader: Read> ExampleService<Reader> {
    pub fn new(reader_queue: Vec<String>, reader_factory: Box<dyn ReaderFactory<Reader>>, parser_settings: ParserSettings) -> ExampleService<Reader> {
        ExampleService {
            reader_queue,
            reader_factory,
            parser_settings,
            active_reader: Default::default(),
            chunk_cache: Default::default(),
        }
    }

    pub async fn read_examples(mut self) -> Result<Vec<Example>, String> {
        while self.reader_queue.len() > 0 {
            self.process_queue();
            self.exhaust_current_queue().await?;
        }

        let mut examples = Vec::new();

        for v in &self.chunk_cache {
            self.verify_example(&v.1)?;

            let mut chunks: Vec<Chunk> = v.1[..].to_vec();

            chunks.sort_by(|lhs, rhs| {
                if let Some(r) = rhs.part_number {
                    if let Some(l) = lhs.part_number {
                        if l < r {
                            return Ordering::Less;
                        } else {
                            return Ordering::Greater;
                        }
                    }
                }

                return Ordering::Equal;
            });

            let content = chunks.into_iter().flat_map(|v| {
                v.content
            }).collect();

            examples.push(Example::new(v.0.clone(), content))
        }

        Ok(examples)
    }

    fn process_queue(&mut self) {
        if let Some(next) = self.reader_queue.pop() {
            let reader = self.reader_factory.make_reader(next);

            self.active_reader = Some(ChunkReader::new(reader, self.parser_settings.clone()));
        } else {
            self.active_reader = None;
        }
    }

    async fn exhaust_current_queue(&mut self) -> Result<(), String> {
        if let Some(active_reader) = &mut self.active_reader {
            while let Some(chunks) = active_reader.next().await {
                let chunks = chunks?;

                for chunk in chunks {
                    let chunk_name = chunk.example_name.clone();

                    let cache = match self.chunk_cache.remove(chunk.example_name.as_str()) {
                        Some(mut cache) => {
                            cache.push(chunk);
                            cache
                        }
                        _ => vec![chunk]
                    };

                    self.chunk_cache.insert(chunk_name, cache);
                }
            }
        }

        Ok(())
    }

    fn verify_example(&self, chunks: &Vec<Chunk>) -> Result<(), String> {
        let mut part_set = HashSet::new();

        for chunk in chunks {
            if let Some(part) = chunk.part_number {
                if part_set.contains(&part) {
                    return Err(format!("Duplicate part {} ", part).into());
                }
                part_set.insert(part);
            } else if chunks.len() > 0 {
                return Err("You must provide a part number for chunk in examples with more than one chunk".into());
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use stringreader::StringReader;

    use crate::layers::domain::reader_factory::ReaderFactory;

    use super::*;

    struct StringReaderFactory {}

    impl ReaderFactory<StringReader<'static>> for StringReaderFactory {
        fn make_reader(&self, name: String) -> StringReader<'static> {
            let content = match name.as_str() {
                "a" => CONTENT_A,
                "b" => CONTENT_B,
                "c" => CONTENT_C,
                "d" => CONTENT_FAIL_D,
                "e" => CONTENT_FAIL_E,
                _ => panic!()
            };

            StringReader::new(content)
        }
    }

    #[tokio::test]
    async fn test_example_producer() {
        let parser_settings = ParserSettings { start_token: "##exemplify-start##".into(), end_token: "##exemplify-end##".into() };

        let mut producer = ExampleService::<StringReader>::new(
            vec!["a".into(), "b".into(), "c".into()],
            Box::new(StringReaderFactory {}),
            parser_settings.clone(),
        );

        let _result = producer.read_examples().await.unwrap();

        let mut producer = ExampleService::<StringReader>::new(
            vec!["d".into()],
            Box::new(StringReaderFactory {}),
            parser_settings.clone(),
        );

        let result = producer.read_examples().await;

        assert_eq!(result.is_err(), true);

        let mut producer = ExampleService::<StringReader>::new(
            vec!["e".into()],
            Box::new(StringReaderFactory {}),
            parser_settings.clone(),
        );

        let result = producer.read_examples().await;

        assert_eq!(result.is_err(), true);
    }

    const CONTENT_A: &str = "\
//##exemplify-start##{name=\"example-1\" part=1}
class ExampleClass {}
//##exemplify-end##
class NotIncludedInExample {}
//##exemplify-start##{name=\"example-1\" part=2}
// This is also part of example-1
//##exemplify-end##
//##exemplify-start##{name=\"example-2\" part=1}
//This chunk has no explicit end
        ";

    const CONTENT_B: &str = "\
//##exemplify-start##{name=\"example-3\" part=1}
class ExampleClass {}
        ";

    const CONTENT_C: &str = "\
//##exemplify-start##{name=\"example-4\"}
class ExampleClass {}
        ";

    const CONTENT_FAIL_D: &str = "\
//##exemplify-start##{name=\"example-5\"}
class ExampleClass {}
//##exemplify-end##
//##exemplify-start##{name=\"example-5\"}
        ";

    const CONTENT_FAIL_E: &str = "\
//##exemplify-start##{name=\"example-5\"}
class ExampleClass {}
//##exemplify-start##{name=\"example-5\"}
        ";
}
