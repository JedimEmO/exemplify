#[macro_use]
extern crate clap;

use std::path::Path;
use std::pin::Pin;
use std::process::exit;

use clap::Clap;
use futures::{Stream, StreamExt};

use exemplify_lib::layers::domain::entities::{Example, Printable};
use exemplify_lib::layers::domain::parser_settings::ParserSettings;
use exemplify_lib::layers::domain::read_examples::read_examples;
use exemplify_lib::layers::domain::reader_stream::reader_stream;
use exemplify_lib::layers::domain::transforms::asciidoctor_transform::{AsciidoctorSettings, map_to_asciidoctor, AsciidoctorExample};
use exemplify_lib::layers::implementations::file_reader_factory::FileReaderFactory;
use exemplify_lib::layers::implementations::fs_discovery::discover_fs_files;

use crate::layers::application::cli_params::{ExemplifyCliParams, OutputFormat};
use crate::layers::application::print_files::print_files;
use futures::executor::block_on_stream;

mod layers;

#[tokio::main]
async fn main() {
    let params: ExemplifyCliParams = ExemplifyCliParams::parse();

    match run(params).await {
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
        _ => {}
    }
}

async fn run(params: ExemplifyCliParams) -> Result<(), String> {
    let files = discover_fs_files(params.source_directory.clone().into(), &params.extensions).unwrap();

    let reader_factory = reader_stream(
        Box::new(FileReaderFactory {}),
        files);

    let parser_settings = ParserSettings { start_token: params.start_token.clone(), end_token: params.end_token.clone() };

    let mut examples = read_examples(reader_factory, parser_settings.clone()).await?;

    match &params.output_format {
        Some(format) => {
            match format {
                OutputFormat::Asciidoctor => {
                    let asciidoc = map_to_asciidoctor(examples, AsciidoctorSettings { callout_token: "".to_string() });

                    print_files(asciidoc, params.clone()).await;
                }
            }
        }
        None => {
            print_files(examples, params.clone()).await;
        }
    }

    Ok(())
}
