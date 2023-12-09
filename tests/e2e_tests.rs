use std::{env, path::PathBuf};

// create test to test ./src/lib.rs
use unrparc::scan_from_reader;

#[test]
fn should_read_medium_rpa_archive_v3_and_return_segments() {
    let path: PathBuf = PathBuf::from(env::current_dir().unwrap())
        .join("tests")
        .join("assets")
        .join("medium_rpa3.rpa");

    // read file using BufReader
    let file = std::fs::File::open(path).unwrap();
    let mut reader = std::io::BufReader::new(file);
    let files = scan_from_reader(&mut reader).unwrap();

    assert_eq!(files.len(), 12);
}

#[test]
fn should_read_big_rpa_archive_v3_and_return_segments() {
    let path = PathBuf::from(env::current_dir().unwrap())
        .join("tests")
        .join("assets")
        .join("big_rpa3.rpa");

    let file = std::fs::File::open(path).unwrap();
    let mut reader = std::io::BufReader::new(file);
    let files = scan_from_reader(&mut reader).unwrap();

    assert_eq!(files.len(), 42);
}
