#[macro_use]
extern crate clap;

use std::path::Path;

use clap::Clap;


use exemplify_lib::layers::domain::parser_settings::ParserSettings;
use exemplify_lib::layers::domain::read_examples::read_examples;
use exemplify_lib::layers::domain::reader_stream::reader_stream;
use exemplify_lib::layers::implementations::file_reader_factory::FileReaderFactory;
use exemplify_lib::layers::implementations::fs_discovery::discover_fs_files;

use crate::layers::application::cli_params::ExemplifyCliParams;

mod layers;

#[tokio::main]
async fn main() {
    let params: ExemplifyCliParams = ExemplifyCliParams::parse();

    let files = discover_fs_files(params.source_directory.clone().into(), &params.extensions).unwrap();

    let reader_factory = reader_stream(
        Box::new(FileReaderFactory{}),
        files);

    let parser_settings = ParserSettings { start_token: params.start_token, end_token: params.end_token };

    let examples = read_examples(reader_factory, parser_settings.clone()).await.unwrap();

    for example in examples {
        let content = example.lines().join("\n");

        if let Some(out_dir) = &params.output_directory {
            let output_path = format!("{}/{}.adoc", out_dir, example.name);
            let output_path = Path::new(&output_path);
            std::fs::create_dir_all(output_path.parent().unwrap()).unwrap();
            std::fs::write(output_path, content).unwrap();
        } else {
            println!("Example {}:\n{}\n", example.name, content);
        }
    }
}
