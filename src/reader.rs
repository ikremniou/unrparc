use std::io::{Seek, Read};

pub(crate) struct RpaBufReader<'a> {
    buf_reader: &'a mut std::io::BufReader<std::fs::File>,
}

pub(crate) trait RpaReader: Read {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64>;
}

impl<'a> RpaBufReader<'a> {
     pub fn new(reader: &mut std::io::BufReader<std::fs::File>) -> RpaBufReader {
        RpaBufReader {
            buf_reader: reader
        }
    }
}

impl<'a> RpaReader for RpaBufReader<'a> {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        return self.buf_reader.seek(pos);
    }
}

impl<'a> Read for RpaBufReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        return self.buf_reader.read(buf);
    }
}

