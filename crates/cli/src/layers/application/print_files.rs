use std::path::Path;
use std::pin::Pin;

use futures::{Stream, StreamExt};

use exemplify_lib::layers::domain::entities::Printable;

use crate::layers::application::cli_params::ExemplifyCliParams;


pub async fn print_files<T: Printable + 'static>(input: Pin<Box<dyn Stream<Item=T>>>, params: ExemplifyCliParams) {
    let _result = input.map(move |example| -> Result<(), String> {
        let content = example.print();

        if let Some(out_dir) = &params.output_folder {
            let output_path = format!("{}/{}", out_dir, example.file_name());
            let output_path = Path::new(&output_path);
            std::fs::create_dir_all(output_path.parent().ok_or("".to_string())?).map_err(|e| e.to_string())?;
            std::fs::write(output_path, content).map_err(|e| e.to_string())?;
        } else {
            println!("Example {}:\n{}\n", example.file_name(), content);
        }

        Ok(())
    }).collect::<Vec<Result<(), String>>>().await;
}
