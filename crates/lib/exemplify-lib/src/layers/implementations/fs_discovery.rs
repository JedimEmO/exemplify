use std::path::Path;
use std::pin::Pin;

use futures::{Stream};

pub fn discover_fs_files(root_folder: String, file_patterns: &Vec<String>) -> Result<Pin<Box<dyn Stream<Item=Result<String, String>>>>, String> {
    let path = Path::new(root_folder.as_str());
    let files = recursively_find_files(path, file_patterns)?;

    Ok(Box::pin(futures::stream::iter(files.into_iter().map(|f| Ok(f)))))
}

fn recursively_find_files(root_folder: &Path, file_patterns: &Vec<String>) -> Result<Vec<String>, String> {
    let mut files: Vec<String> = Vec::new();

    for entry in std::fs::read_dir(root_folder).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;

        let path = entry.path();

        let metadata = std::fs::symlink_metadata(&path).map_err(|e| e.to_string())?;

        if metadata.file_type().is_symlink() {
            continue;
        }

        if path.is_file() {
            for ext in file_patterns {
                let string_path = path.to_str().ok_or("".to_string())?.to_string();

                if path.extension().ok_or("Invalid filename")?.to_str().ok_or("Invalid filename")? == ext {
                    files.push(string_path)
                }
            }
        } else if path.is_dir() {
            let mut sub_folder_files = recursively_find_files(&entry.path(), file_patterns)?;

            files.append(&mut sub_folder_files);
        }
    }

    Ok(files)
}
