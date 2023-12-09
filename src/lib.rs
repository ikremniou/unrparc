use std::{
    collections::BTreeMap,
    io::{BufReader, Read, Seek},
};

use error::UnrparcError;
use flate2::bufread::ZlibDecoder;

mod error;

pub struct File {
    pub name: String,
    pub offset: i64,
    pub size: i64,
}

pub fn scan_from_reader(reader: &mut BufReader<std::fs::File>) -> Result<Vec<File>, UnrparcError> {
    let file_size = reader.seek(std::io::SeekFrom::End(0))?;
    reader.seek(std::io::SeekFrom::Start(0))?;

    let mut bytes = [0u8; 10];
    reader.read_exact(&mut bytes)?;
    reader.seek(std::io::SeekFrom::Start(0))?;
    if bytes.starts_with(b"RPA-3.0 ") {
        return scan_rpa(reader, file_size);
    }

    return single_segment_file(reader, file_size);
}

fn scan_rpa(
    reader: &mut BufReader<std::fs::File>,
    _file_size: u64,
) -> Result<Vec<File>, UnrparcError> {
    let mut header = [0u8; 40];
    reader.read_exact(&mut header)?;

    let offset = u64::from_str_radix(&String::from_utf8_lossy(&header[8..24]), 16)?;
    let key = u32::from_str_radix(&String::from_utf8_lossy(&header[25..33]), 16)?;
    reader.seek(std::io::SeekFrom::Start(offset))?;

    let mut buf = Vec::new();
    ZlibDecoder::new(reader).read_to_end(&mut buf)?;
    let index_raw: serde_pickle::Value = serde_pickle::from_slice(&buf, Default::default())?;

    let index = match index_raw {
        serde_pickle::Value::Dict(index) => index,
        _ => return Err(error::UnrparcError),
    };

    return fetch_files_from_index(index, key);
}

fn fetch_files_from_index(
    index: BTreeMap<serde_pickle::HashableValue, serde_pickle::Value>,
    key: u32,
) -> Result<Vec<File>, UnrparcError> {
    let mut files = Vec::new();
    for (file_name, value) in index {
        let real_val = match value {
            serde_pickle::Value::List(ref v) => match &v[0] {
                serde_pickle::Value::List(v) => v,
                _ => return Err(error::UnrparcError),
            },
            _ => return Err(error::UnrparcError),
        };

        let offset = match real_val[0] {
            serde_pickle::Value::I64(offset) => offset,
            _ => return Err(error::UnrparcError),
        };

        let size = match real_val[1] {
            serde_pickle::Value::I64(size) => size,
            _ => return Err(error::UnrparcError),
        };

        files.push(File {
            name: file_name.to_string(),
            offset: offset ^ key as i64,
            size: size ^ key as i64,
        });
    }
    return Ok(files);
}

fn single_segment_file(
    _reader: &mut BufReader<std::fs::File>,
    file_size: u64,
) -> Result<Vec<File>, UnrparcError> {
    let mut result = Vec::new();
    result.push(File {
        name: String::from("default"),
        offset: 0,
        size: file_size as i64,
    });
    return Ok(result);
}
