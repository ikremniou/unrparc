use std::{collections::BTreeMap, io::Read};

use flate2::read::ZlibDecoder;

use crate::{error::UnrparcError, reader::RpaReader, RpaFile};

pub(crate) fn scan_rpa(reader: &mut dyn RpaReader) -> Result<Vec<RpaFile>, UnrparcError> {
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
        _ => return Err(UnrparcError),
    };

    fetch_files_from_index(index, key)
}

fn fetch_files_from_index(
    index: BTreeMap<serde_pickle::HashableValue, serde_pickle::Value>,
    key: u32,
) -> Result<Vec<RpaFile>, UnrparcError> {
    let mut files = Vec::new();
    for (file_name, value) in index {
        let real_val = match value {
            serde_pickle::Value::List(ref v) => match &v[0] {
                serde_pickle::Value::List(v) => v,
                _ => return Err(UnrparcError),
            },
            _ => return Err(UnrparcError),
        };

        let offset = match real_val[0] {
            serde_pickle::Value::I64(offset) => offset,
            _ => return Err(UnrparcError),
        };

        let size = match real_val[1] {
            serde_pickle::Value::I64(size) => size,
            _ => return Err(UnrparcError),
        };

        let real_name = match file_name {
            serde_pickle::HashableValue::String(ref name) => name,
            _ => return Err(UnrparcError),
        };

        files.push(RpaFile {
            name: real_name.clone(),
            offset: offset ^ key as i64,
            size: size ^ key as i64,
        });
    }
    Ok(files)
}

pub(crate) fn extract_file(
    file: RpaFile,
    reader: &mut dyn RpaReader,
) -> Result<(RpaFile, Vec<u8>), UnrparcError> {
    reader.seek(std::io::SeekFrom::Start(file.offset as u64))?;

    let mut buf = vec![0u8; file.size as usize];
    reader.read_exact(&mut buf)?;

    Ok((file, buf))
}

pub(crate) fn extract_predicate<F>(
    predicate: F,
    reader: &mut dyn RpaReader,
) -> Result<Vec<(RpaFile, Vec<u8>)>, UnrparcError>
where
    F: Fn(&RpaFile) -> bool,
{
    let files = scan_rpa(reader)?;
    if files.is_empty() {
        return Err(UnrparcError);
    }

    let mut result_files: Vec<(RpaFile, Vec<u8>)> = Vec::new();
    for file in files {
        if predicate(&file) {
            result_files.push(extract_file(file, reader)?);
        }
    }

    Ok(result_files)
}
