use std::{fs::create_dir_all, path::Path};

pub fn prepare_out_dir(out_dir: &Path) {
    if !out_dir.exists() {
        create_dir_all(out_dir)
            .expect("Expected to create a directory.");
    }
}

pub fn get_file_name(path: &Path) -> &str {
    path
        .file_name()
        .expect("Expected file to have a name.")
        .to_str()
        .expect("Expected file to have a valid UTF-8 name.")
}
