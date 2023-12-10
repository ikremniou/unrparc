use std::{env, io::BufReader, path::PathBuf};

use unrparc::{extract_file, extract_filename, extract_glob, scan, RpaFile};

fn read_assets_rpa(name: &str) -> (Vec<RpaFile>, BufReader<std::fs::File>) {
    let path: PathBuf = env::current_dir().unwrap()
        .join("tests")
        .join("assets")
        .join(name);

    let file = std::fs::File::open(path).unwrap();
    let mut reader = std::io::BufReader::new(file);
    (scan(&mut reader).unwrap(), reader)
}

#[test]
fn should_scan_medium_rpa_archive_and_return_files_with_correct_size() {
    let files = read_assets_rpa("medium_rpa3.rpa").0;

    assert_eq!(files.len(), 12);
    assert_eq!(files[0].offset, 51);
    assert_eq!(files[0].size, 134436);
}

#[test]
fn should_scan_big_rpa_and_verify_returned_offsets() {
    let files = read_assets_rpa("big_rpa3.rpa").0;

    assert_eq!(files.len(), 42);
    assert_eq!(files[1].name, "cgs.rpyc");

    let mut prev_offset: i64 = -1;
    for file in files {
        assert!(file.offset > prev_offset);
        prev_offset = file.offset;
    }
}

#[test]
fn should_scan_archive_with_single_file_and_return_one_readme() {
    let files = read_assets_rpa("bonus.rpa").0;

    assert_eq!(files.len(), 1);
}

#[test]
fn should_scan_archive_with_multiple_scripts_and_return_many_files() {
    let files = read_assets_rpa("scripts.rpa").0;

    assert_eq!(files.len(), 8);
}

#[test]
fn should_extract_single_file_by_name() {
    let (files, mut reader) = read_assets_rpa("bonus.rpa");

    let file = extract_filename(files[0].name.as_str(), &mut reader).unwrap();

    assert_eq!(file.1, b"1234567890");
}

#[test]
fn should_extract_single_file_by_file() {
    let (files, mut reader) = read_assets_rpa("bonus.rpa");

    let file = extract_file(files[0].clone(), &mut reader).unwrap();

    assert_eq!(file.1, b"1234567890");
}

#[test]
fn should_extract_all_files_by_specific_glob() {
    let (_, mut reader) = read_assets_rpa("scripts.rpa");

    let files = extract_glob("*.rpy", &mut reader).unwrap();

    for file in files {
        assert!(file.0.name.ends_with(".rpy"));
    }
}
