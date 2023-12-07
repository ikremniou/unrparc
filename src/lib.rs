use std::io::{BufReader, Read, Seek};

use error::UnrparcError;

mod error;

pub fn hello() -> String {
    "Hello, World!".to_string()
}

pub fn scan_rpa(reader: &mut BufReader<std::fs::File>) -> Result<(), UnrparcError> {
    let mut bytes = [0u8; 1024];

    reader.read_exact(&mut bytes)?;

    let file_size = reader.seek(std::io::SeekFrom::End(0))?;
    print!("File size: {file_size} bytes\n");

    return Ok(());
}
