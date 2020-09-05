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

pub fn map_to_asciidoctor(input: Pin<Box<dyn Stream<Item=Example>>>, settings: AsciidoctorSettings) -> Pin<Box<dyn Stream<Item=AsciidoctorExample>>> {
    Box::pin(input.map(move |example| {
        let header = create_asciidoc_source_header(&settings, &example);
        let footer = create_asciidoc_source_footer(&settings);

        AsciidoctorExample {
            inner: Example {
                name: example.name,
                content: vec![
                    header,
                    example.content,
                    footer
                ].into_iter().flatten().collect(),
                title: example.title,
                language: example.language,
            }
        }
    }))
}

fn create_asciidoc_source_header(_settings: &AsciidoctorSettings, example: &Example) -> Vec<String> {
    let title = match &example.title {
        Some(title) => vec![format!(".{}", title)],
        _ => vec![]
    };

    vec![
        title,
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
