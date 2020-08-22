mod utils;

use rayon::prelude::*;
use sc_extract::process_tex;
use std::{fs, path::Path};
use utils::*;

#[test]
fn test_single() {
    let path = Path::new("./tests/data/sc/background_basic_tex.sc");
    let data = fs::read(&path).unwrap();
    let out_dir = Path::new("./tests/out/sc");

    prepare_out_dir(&out_dir);

    assert_eq!(
        true,
        process_tex(data.as_slice(), get_file_name(path), &out_dir, true).is_ok()
    );
}

#[test]
fn test_all_parallel() {
    let dir = Path::new("./tests/data/sc");
    let out_dir = Path::new("./tests/out/sc");

    prepare_out_dir(&out_dir);

    let dir_entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => std::process::exit(1),
    };

    let mut entries = Vec::new();
    for entry in dir_entries {
        entries.push(entry);
    }

    entries.into_par_iter().for_each(|entry| {
        let path = entry.unwrap().path();
        let data = fs::read(&path).unwrap();
        assert_eq!(
            true,
            process_tex(data.as_slice(), get_file_name(&path), &out_dir, true).is_ok()
        );
    });
}

#[test]
fn test_all_blocking() {
    let dir = Path::new("./tests/data/sc");
    let out_dir = Path::new("./tests/out/sc");

    prepare_out_dir(&out_dir);

    let dir_entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => std::process::exit(1),
    };

    let mut entries = Vec::new();
    for entry in dir_entries {
        entries.push(entry);
    }

    for entry in entries {
        let path = entry.unwrap().path();
        let data = fs::read(&path).unwrap();
        assert_eq!(
            true,
            process_tex(data.as_slice(), get_file_name(&path), &out_dir, false).is_ok()
        );
    }
}
