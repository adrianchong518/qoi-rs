use crate::{Error, Result};

pub trait Writer {
    fn write_byte(&mut self, byte: u8) -> Result<usize> {
        self.write_from_slice(&[byte])
    }

    fn write_from_slice(&mut self, bytes: &[u8]) -> Result<usize>;
}

impl<T: std::io::Write> Writer for T {
    fn write_from_slice(&mut self, bytes: &[u8]) -> Result<usize> {
        self.write_all(bytes)
            .map(|_| bytes.len())
            .map_err(|err| Error::IoError(err.kind()))
    }
}
