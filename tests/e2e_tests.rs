use std::{env, path::PathBuf};

// create test to test ./src/lib.rs
use unrparc::{hello, scan_rpa};

#[test]
fn should_read_small_rpa_archive_v3_and_return_segments() {
    let path: PathBuf = PathBuf::from(env::current_dir().unwrap())
        .join("tests")
        .join("assets")
        .join("medium_rpa3.rpa");


    // read file using BufReader
    let file = std::fs::File::open(path).unwrap();
    let mut reader = std::io::BufReader::new(file);
    scan_rpa(&mut reader).unwrap();
}

#[test]
fn should_read_medium_rpa_archive_v3_and_return_segments() {
    hello();
}

#[test]
fn should_read_small_rpa_archive_2005_and_return_segments() {
    hello();
}
