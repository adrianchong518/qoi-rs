use crate::{
    header::{ColorChannel, ColorSpace, Header},
    pixel::{Pixel, SupportedChannels},
    writer::Writer,
    Error, Result,
};

pub fn encode<const N: usize>(
    writer: &mut impl Writer,
    pixels: &[Pixel<N>],
    width: u32,
    height: u32,
    color_space: ColorSpace,
) -> Result<usize>
where
    Pixel<N>: SupportedChannels,
{
    let image_size = (width as usize).saturating_mul(height as usize);
    if pixels.len() != image_size {
        return Err(Error::UnmatchedDataSize {
            data_size: pixels.len(),
            header_size: image_size,
        });
    }

    let mut written = 0;

    // Write header information
    {
        let channels = match N {
            3 => ColorChannel::Rgb,
            4 => ColorChannel::Rgba,
            _ => unreachable!(),
        };

        let header = Header::new(width, height, channels, color_space);
        written += writer.write_from_slice(&header.as_bytes())?;
    }

    Ok(written)
}
