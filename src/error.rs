pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidHeaderSize(usize),
    InvalidMagic([u8; 4]),
    InvalidChannelNumber(u8),
    InvalidColorspace(u8),

    UnmatchedDataSize {
        data_size: usize,
        header_size: usize,
    },

    IoError(std::io::ErrorKind),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}
