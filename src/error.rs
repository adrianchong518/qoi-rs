//! Errors for the library

/// A convenient short hand for `Result`s with our [`Error`] type
pub type Result<T> = std::result::Result<T, Error>;

/// Possible errors
#[derive(Debug)]
pub enum Error {
    /// Did not find magic bytes `b"qoif"`
    InvalidMagic([u8; 4]),

    /// Invalid number of channels
    InvalidChannelNumber(u8),

    /// Invalid color space ID
    InvalidColorSpace(u8),

    /// `data_size` does not match metadata (`header_size`)
    UnmatchedDataSize {
        data_size: usize,
        header_size: usize,
    },

    /// Wrapper for `std::io::Error`
    IoError(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}
