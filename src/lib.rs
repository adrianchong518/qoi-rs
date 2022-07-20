pub mod io;

mod constants;
mod encode;
mod error;
mod header;
mod pixel;

#[macro_use]
extern crate num_derive;

pub use encode::encode;
pub use error::{Error, Result};
pub use header::{ColorChannel, ColorSpace, Header};
pub use pixel::Pixel;
