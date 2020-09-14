use std::pin::Pin;

use futures::{Stream, StreamExt};
use crate::layers::domain::entities::{Example, Printable};

pub struct AsciidoctorSettings {
    pub callout_token: String
}

pub struct AsciidoctorExample {
    inner: Example
}

impl Printable for AsciidoctorExample {
    fn print(&self) -> String {
        self.inner.print()
    }

    fn file_name(&self) -> String {
        format!("{}.adoc", self.inner.name)
    }
}

pub fn map_to_asciidoctor(input: Pin<Box<dyn Stream<Item=Example>>>, settings: AsciidoctorSettings) -> Pin<Box<dyn Stream<Item=Result<AsciidoctorExample, String>>>> {
    Box::pin(input.map(move |example| {
        let header = create_asciidoc_source_header(&settings, &example);
        let footer = create_asciidoc_source_footer(&settings);

        let content = transform_callouts(example.content, &settings.callout_token)?;
        let callouts = content.1
            .into_iter()
            .map(|callout| format!("<{}> {}", callout.number, callout.text))
            .collect();

        Ok(AsciidoctorExample {
            inner: Example {
                name: example.name,
                content: vec![
                    header,
                    content.0,
                    footer,
                    callouts
                ].into_iter().flatten().collect(),
                title: example.title,
                language: example.language,
                id: example.id
            }
        })
    }))
}

fn create_asciidoc_source_header(_settings: &AsciidoctorSettings, example: &Example) -> Vec<String> {
    let title = match &example.title {
        Some(title) => vec![format!(".{}", title)],
        _ => vec![]
    };

    let id = match &example.id {
        Some(id) => vec![format!("[#{}]", id)],
        _  => vec![]
    };

    vec![
        title,
        id,
        vec![format!("[source{}]", match &example.language {
            Some(language) => format!(",{}", language),
            _ => "".into()
        })],
        vec!["----".into()]
    ].into_iter().flatten().collect()
}

fn create_asciidoc_source_footer(_settings: &AsciidoctorSettings) -> Vec<String> {
    vec![
        "----".into()
    ]
}

struct Callout {
    text: String,
    number: usize
}

fn transform_callouts(input: Vec<String>, callout_token: &String) -> Result<(Vec<String>, Vec<Callout>), String> {
    let mut callout_number = 1;
    let mut output = Vec::new();
    let mut callouts= Vec::new();

    for mut line in input {
        loop {
            if line.contains(callout_token) {
                line = line.replacen(callout_token, format!("<{}>", callout_number).as_str(), 1);
                let extract = extract_first_callout(line, callout_number)
                    .map_err(|e| format!("Failed extracting callout from {}", e))?;

                line = extract.0;
                callouts.push(extract.1);

                callout_number += 1;
            } else {
                break;
            }
        }

        output.push(line);
    }

    Ok((output, callouts))
}

fn extract_first_callout(mut input: String, idx: usize) -> Result<(String, Callout), String> {
    lazy_static::lazy_static! {
        static ref CALLOUT_RE: regex::Regex = regex::Regex::new("\\{value=\"(.+)\"\\}").unwrap();
    }

    for val in CALLOUT_RE.captures_iter(input.clone().as_str()) {
        let value = val.get(1);

        if let Some(value) = value {
            input = CALLOUT_RE.replace(input.as_str(), "").to_string();

            return Ok((input, Callout {
                text: value.as_str().to_string().clone(),
                number: idx
            }))
        }
    }

    Err(input)
}
