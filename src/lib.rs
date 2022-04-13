mod constants;
mod encode;
mod error;
mod header;
mod pixel;
mod writer;

#[macro_use]
extern crate num_derive;

pub use encode::encode;
pub use error::{Error, Result};
