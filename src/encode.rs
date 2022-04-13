use crate::{
    constants::{
        QOI_END_MARKER, QOI_OP_DIFF, QOI_OP_INDEX, QOI_OP_LUMA, QOI_OP_RGB, QOI_OP_RGBA, QOI_OP_RUN,
    },
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
    // Ensure size of image data provided is the same as the provided dimensions
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

    let mut previous_pixel = Pixel::<N>::new_initial();
    let mut seen_pixels = [Pixel::<4>::default(); 64];
    let mut run = 0u8;

    fn emit_qoi_op_run(w: &mut impl Writer, run: &mut u8) -> Result<usize> {
        debug_assert!(*run > 0);

        let written = w.write_byte(QOI_OP_RUN | (*run - 1))?;
        *run = 0;

        Ok(written)
    }

    for pixel in pixels {
        (|| -> Result<()> {
            // Check if the previous pixel is the same
            if *pixel == previous_pixel {
                run += 1;

                // NB: Maximum possible run is `62`
                if run == 62 {
                    written += emit_qoi_op_run(writer, &mut run)?;
                }

                return Ok(());
            }

            // Emit a QOI_OP_RUN if there is an existing run of same pixels
            // NB: This will **NOT** return early as the current `pixel` is not handled yet
            if run > 0 {
                written += emit_qoi_op_run(writer, &mut run)?;
            }

            let index = pixel.index_hash();

            // Check if the current `pixel` can be indexed in the array
            if pixel.as_rgba() == seen_pixels[index] {
                written += writer.write_byte(QOI_OP_INDEX | index as u8)?;
                return Ok(());
            }

            // Update the seem pixel
            seen_pixels[index] = pixel.as_rgba();

            // If the alpha channel of the pixel is different, there is no choice but to emit a
            // `QOI_OP_RGBA`
            // NB: This only matters if there is alpha channel data, ie `N == 4`
            if N == 4 && pixel.alpha() != previous_pixel.alpha() {
                written += writer.write_byte(QOI_OP_RGBA)?;
                written += writer.write_from_slice(&pixel.as_inner_rgba())?;
                return Ok(());
            }

            // Calculate the difference for each channels
            let diff_red = pixel.red().wrapping_sub(previous_pixel.red());
            let diff_green = pixel.green().wrapping_sub(previous_pixel.green());
            let diff_blue = pixel.blue().wrapping_sub(previous_pixel.blue());

            // Attempt to use `QOI_OP_DIFF`
            {
                // Bias the differences by `2`
                let diff_red = diff_red.wrapping_add(2);
                let diff_green = diff_green.wrapping_add(2);
                let diff_blue = diff_blue.wrapping_add(2);

                // NB: Maximum biased difference for each channel is `3`
                if diff_red <= 3 && diff_green <= 3 && diff_blue <= 3 {
                    written += writer
                        .write_byte(QOI_OP_DIFF | diff_red << 4 | diff_green << 2 | diff_blue)?;

                    return Ok(());
                }
            }

            let diff_red_green = diff_red.wrapping_sub(diff_green);
            let diff_blue_green = diff_blue.wrapping_sub(diff_green);

            // Attempt to use `QOI_OP_LUMA`
            {
                let diff_green = diff_green.wrapping_add(32);
                let diff_red_green = diff_red_green.wrapping_add(8);
                let diff_blue_green = diff_blue_green.wrapping_add(8);

                // NB: Maximum biased differences are 63 for green and 15 for both "red-green" and
                // NB: "blue-green"
                if diff_green <= 63 && diff_red_green <= 15 && diff_blue_green <= 15 {
                    written += writer.write_from_slice(&[
                        QOI_OP_LUMA | diff_green,
                        diff_red_green << 4 | diff_blue_green,
                    ])?;

                    return Ok(());
                }
            }

            // Final fall-through case: emit a `QOI_OP_RGB`
            {
                written += writer.write_byte(QOI_OP_RGB)?;
                written += writer.write_from_slice(&pixel.as_inner_rgb())?;
            }

            Ok(())
        })()?;

        // Update previous pixel
        previous_pixel = *pixel;
    }

    // Emit a last `QOI_OP_RUN` if there is a remaining run at the end
    if run > 0 {
        written += emit_qoi_op_run(writer, &mut run)?;
    }

    // Write the end marker
    written += writer.write_from_slice(QOI_END_MARKER)?;

    Ok(written)
}

#[cfg(test)]
mod tests {
    use crate::{encode, header::ColorSpace, pixel::Pixel};

    #[test]
    fn can_encode_rgb() {
        let pixels = [
            Pixel::rgb(100, 100, 100),
            Pixel::rgb(200, 200, 200),
            Pixel::rgb(100, 101, 100),
        ];
        let width = 3;
        let height = 1;
        let color_space = ColorSpace::AllLinear;

        let mut buf = vec![];

        let result = encode(&mut buf, &pixels, width, height, color_space);

        assert!(matches!(result, Ok(34)), "result unmatched: {result:?}");

        assert_eq!(
            buf,
            [
                0x71, 0x6f, 0x69, 0x66, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x01, 0x03, 0x01,
                0xfe, 0x64, 0x64, 0x64, 0xfe, 0xc8, 0xc8, 0xc8, 0xfe, 0x64, 0x65, 0x64, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x01
            ],
        );
    }

    #[test]
    fn can_encode_rgba() {
        let pixels = [
            Pixel::rgba(100, 100, 100, 200),
            Pixel::rgba(200, 200, 200, 100),
            Pixel::rgba(100, 101, 100, 255),
        ];
        let width = 3;
        let height = 1;
        let color_space = ColorSpace::AllLinear;

        let mut buf = vec![];

        let result = encode(&mut buf, &pixels, width, height, color_space);

        assert!(matches!(result, Ok(37)), "result unmatched: {result:?}");

        assert_eq!(
            buf,
            [
                0x71, 0x6f, 0x69, 0x66, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x01, 0x04, 0x01,
                0xff, 0x64, 0x64, 0x64, 0xc8, 0xff, 0xc8, 0xc8, 0xc8, 0x64, 0xff, 0x64, 0x65, 0x64,
                0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01
            ],
        );
    }

    #[test]
    fn can_encode_mixed_rgba() {
        let pixels = [
            Pixel::rgba(100, 100, 100, 200),
            Pixel::rgba(200, 200, 200, 100),
            Pixel::rgba(100, 101, 100, 100),
            Pixel::rgba(100, 101, 100, 255),
        ];
        let width = 4;
        let height = 1;
        let color_space = ColorSpace::Srgb;

        let mut buf = vec![];

        let result = encode(&mut buf, &pixels, width, height, color_space);

        assert!(matches!(result, Ok(41)), "result unmatched: {result:?}");

        assert_eq!(
            buf,
            [
                0x71, 0x6f, 0x69, 0x66, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x01, 0x04, 0x00,
                0xff, 0x64, 0x64, 0x64, 0xc8, 0xff, 0xc8, 0xc8, 0xc8, 0x64, 0xfe, 0x64, 0x65, 0x64,
                0xff, 0x64, 0x65, 0x64, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01
            ],
        );
    }

    #[test]
    fn can_encode_index() {
        let pixels = [
            Pixel::rgb(100, 100, 100),
            Pixel::rgb(200, 200, 200),
            Pixel::rgb(100, 100, 100),
            Pixel::rgb(0, 0, 0),
            Pixel::rgb(200, 200, 200),
            Pixel::rgb(0, 0, 0),
        ];
        let width = 3;
        let height = 2;
        let color_space = ColorSpace::AllLinear;

        let mut buf = vec![];

        let result = encode(&mut buf, &pixels, width, height, color_space);

        assert!(matches!(result, Ok(37)), "result unmatched: {result:?}");

        assert_eq!(
            buf,
            [
                0x71, 0x6f, 0x69, 0x66, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x02, 0x03, 0x01,
                0xfe, 0x64, 0x64, 0x64, 0xfe, 0xc8, 0xc8, 0xc8, 0x11, 0xfe, 0x00, 0x00, 0x00, 0x2d,
                0x35, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01
            ],
        );
    }

    #[test]
    fn can_encode_without_repeating_index() {
        let pixels = [
            Pixel::rgba(100, 100, 100, 100),
            Pixel::rgba(200, 200, 200, 255),
            Pixel::rgba(100, 100, 100, 100),
            Pixel::rgba(100, 100, 100, 100),
            Pixel::rgba(100, 100, 100, 100),
            Pixel::rgba(100, 100, 100, 100),
            Pixel::rgba(100, 100, 100, 100),
            Pixel::rgba(100, 100, 100, 100),
            Pixel::rgba(100, 100, 100, 100),
        ];
        let width = 3;
        let height = 3;
        let color_space = ColorSpace::AllLinear;

        let mut buf = vec![];

        let result = encode(&mut buf, &pixels, width, height, color_space);

        assert!(matches!(result, Ok(34)), "result unmatched: {result:?}");

        assert_eq!(
            buf,
            [
                0x71, 0x6f, 0x69, 0x66, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x03, 0x04, 0x01,
                0xff, 0x64, 0x64, 0x64, 0x64, 0xff, 0xc8, 0xc8, 0xc8, 0xff, 0x28, 0xc5, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x01
            ],
        );
    }

    #[test]
    fn can_encode_diff() {
        let pixels = [
            Pixel::rgb(1, 1, 1),
            Pixel::rgb(2, 2, 2),
            Pixel::rgb(0, 0, 0),
            Pixel::rgb(255, 255, 255),
        ];
        let width = 2;
        let height = 2;
        let color_space = ColorSpace::AllLinear;

        let mut buf = vec![];

        let result = encode(&mut buf, &pixels, width, height, color_space);

        assert!(matches!(result, Ok(26)), "result unmatched: {result:?}");

        assert_eq!(
            buf,
            [
                0x71, 0x6f, 0x69, 0x66, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x02, 0x03, 0x01,
                0x7f, 0x7f, 0x40, 0x55, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01
            ],
        );
    }

    #[test]
    fn can_encode_luma() {
        let pixels = [
            Pixel::rgb(25, 30, 35),
            Pixel::rgb(20, 15, 3),
            Pixel::rgb(36, 29, 17),
            Pixel::rgb(33, 30, 25),
        ];
        let width = 2;
        let height = 2;
        let color_space = ColorSpace::AllLinear;

        let mut buf = vec![];

        let result = encode(&mut buf, &pixels, width, height, color_space);

        assert!(matches!(result, Ok(32)), "result unmatched: {result:?}");

        assert_eq!(
            buf,
            [
                0x71, 0x6f, 0x69, 0x66, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x02, 0x03, 0x01,
                0xbe, 0x3d, 0xfe, 0x14, 0x0f, 0x03, 0xae, 0xa8, 0xa1, 0x4f, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x01
            ],
        );
    }

    #[test]
    fn can_encode_run() {
        let pixels = [Pixel::rgb(127, 127, 127); 20];
        let width = 5;
        let height = 4;
        let color_space = ColorSpace::AllLinear;

        let mut buf = vec![];

        let result = encode(&mut buf, &pixels, width, height, color_space);

        assert!(matches!(result, Ok(27)), "result unmatched: {result:?}");

        assert_eq!(
            buf,
            [
                0x71, 0x6f, 0x69, 0x66, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x04, 0x03, 0x01,
                0xfe, 0x7f, 0x7f, 0x7f, 0xd2, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01
            ],
        );
    }
}
