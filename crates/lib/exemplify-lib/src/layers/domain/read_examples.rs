use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::io::Read;
use std::pin::Pin;

use futures::{Stream, StreamExt};

use crate::layers::domain::chunk::Chunk;
use crate::layers::domain::chunk_reader::ChunkReader;
use crate::layers::domain::parser_settings::ParserSettings;

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

pub async fn read_examples<Reader: Read>(mut reader_factory: Pin<Box<dyn Stream<Item=Result<Reader, String>>>>, parser_settings: ParserSettings) -> Result<Vec<Example>, String> {
    let mut chunk_cache: HashMap<String, Vec<Chunk>> = Default::default();

    while let Some(reader) = reader_factory.next().await {
        let chunk_reader = ChunkReader::new(reader?, parser_settings.clone());

        chunk_cache = exhaust_reader(chunk_reader, chunk_cache).await?;
    }

    finalize_examples(chunk_cache)
}

fn finalize_examples(mut chunk_cache: HashMap<String, Vec<Chunk>>) -> Result<Vec<Example>, String> {
    let mut examples = Vec::new();

    for v in &chunk_cache {
        verify_example(&v.1)?;

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

async fn exhaust_reader<Reader: Read>(mut chunk_reader: ChunkReader<Reader>, mut chunk_cache: HashMap<String, Vec<Chunk>>) -> Result<HashMap<String, Vec<Chunk>>, String> {
    while let Some(chunks) = chunk_reader.next().await {
        let chunks = chunks?;

        for chunk in chunks {
            let chunk_name = chunk.example_name.clone();

            let cache = match chunk_cache.remove(chunk.example_name.as_str()) {
                Some(mut cache) => {
                    cache.push(chunk);
                    cache
                }
                _ => vec![chunk]
            };

            chunk_cache.insert(chunk_name, cache);
        }
    }

    Ok(chunk_cache)
}

fn verify_example(chunks: &Vec<Chunk>) -> Result<(), String> {
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


impl Example {
    pub fn lines(&self) -> &Vec<String> {
        &self.content
    }
}

#[cfg(test)]
mod test {
    use std::rc::Rc;

    use stringreader::StringReader;

    use crate::layers::domain::reader_factory::ReaderFactory;

    use super::*;
    use crate::layers::domain::reader_stream::reader_stream;

    struct StringReaderFactory {}

    impl ReaderFactory<StringReader<'static>> for StringReaderFactory {
        fn make_reader(&self, name: String) -> Result<StringReader<'static>, String> {
            let content = match name.as_str() {
                "a" => CONTENT_A,
                "b" => CONTENT_B,
                "c" => CONTENT_C,
                "d" => CONTENT_FAIL_D,
                "e" => CONTENT_FAIL_E,
                _ => panic!()
            };

            Ok(StringReader::new(content))
        }
    }

    #[tokio::test]
    async fn test_example_producer() {
        let parser_settings = ParserSettings { start_token: "##exemplify-start##".into(), end_token: "##exemplify-end##".into() };

        let file_name_stream = Box::pin(futures::stream::iter(
            vec![
                Ok("a".into()),
                Ok("b".into()),
                Ok("c".into())
            ].into_iter()));

        let file_reader_factory = reader_stream(Box::new(StringReaderFactory {}), file_name_stream);
        let _result = read_examples(file_reader_factory, parser_settings.clone()).await.unwrap();

        let file_name_stream = Box::pin(futures::stream::iter(
            vec![
                Ok("d".into())
            ].into_iter()));

        let file_reader_factory = reader_stream(Box::new(StringReaderFactory {}), file_name_stream);
        let result = read_examples(file_reader_factory, parser_settings.clone()).await;

        assert_eq!(result.is_err(), true);

        let file_name_stream = Box::pin(futures::stream::iter(
            vec![
                Ok("e".into())
            ].into_iter()));

        let file_reader_factory = reader_stream(Box::new(StringReaderFactory {}), file_name_stream);
        let result = read_examples(file_reader_factory, parser_settings.clone()).await;

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
