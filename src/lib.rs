use std::io::BufReader;

use error::UnrparcError;
use reader::RpaReader;
use wax::{Glob, Pattern};

mod error;
mod internal;
mod reader;

#[derive(Clone)]
pub struct RpaFile {
    pub name: String,
    pub offset: i64,
    pub size: i64,
}

pub fn scan(reader: &mut BufReader<std::fs::File>) -> Result<Vec<RpaFile>, UnrparcError> {
    let mut rpa_reader: Box<dyn RpaReader> = Box::new(reader::RpaBufReader::new(reader));

    let mut bytes = [0u8; 10];
    rpa_reader.as_mut().read_exact(&mut bytes)?;
    rpa_reader.as_mut().seek(std::io::SeekFrom::Start(0))?;
    if bytes.starts_with(b"RPA-3.0 ") {
        return internal::scan_rpa(rpa_reader.as_mut());
    }

    Err(UnrparcError)
}

pub fn extract(
    reader: &mut BufReader<std::fs::File>,
) -> Result<Vec<(RpaFile, Vec<u8>)>, UnrparcError> {
    let mut rpa_reader: Box<dyn RpaReader> = Box::new(reader::RpaBufReader::new(reader));
    rpa_reader.as_mut().seek(std::io::SeekFrom::Start(0))?;
    return internal::extract_predicate(|_| true, rpa_reader.as_mut());
}

pub fn extract_filename(
    file_name: &str,
    reader: &mut BufReader<std::fs::File>,
) -> Result<(RpaFile, Vec<u8>), UnrparcError> {
    let mut rpa_reader: Box<dyn RpaReader> = Box::new(reader::RpaBufReader::new(reader));

    rpa_reader.as_mut().seek(std::io::SeekFrom::Start(0))?;

    let files = internal::extract_predicate(|file| file.name == file_name, rpa_reader.as_mut())?;
    Ok(files[0].clone())
}

pub fn extract_glob(
    glob: &str,
    reader: &mut BufReader<std::fs::File>,
) -> Result<Vec<(RpaFile, Vec<u8>)>, UnrparcError> {
    let mut rpa_reader: Box<dyn RpaReader> = Box::new(reader::RpaBufReader::new(reader));
    rpa_reader.as_mut().seek(std::io::SeekFrom::Start(0))?;

    let glob = Glob::new(glob)?;
    return internal::extract_predicate(
        |file| glob.is_match(file.name.as_str()),
        rpa_reader.as_mut(),
    );
}

pub fn extract_file(
    file: RpaFile,
    reader: &mut BufReader<std::fs::File>,
) -> Result<(RpaFile, Vec<u8>), UnrparcError> {
    let mut rpa_reader: Box<dyn RpaReader> = Box::new(reader::RpaBufReader::new(reader));
    return internal::extract_file(file, rpa_reader.as_mut());
}
