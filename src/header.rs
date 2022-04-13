use num::FromPrimitive;

use crate::{constants::QOI_MAGIC, Error, Result};

#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
#[repr(u8)]
pub enum ColorChannel {
    Rgb = 3,
    Rgba = 4,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
#[repr(u8)]
pub enum ColorSpace {
    Srgb = 0,
    AllLinear = 1,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Header {
    width: u32,
    height: u32,
    channels: ColorChannel,
    color_space: ColorSpace,
}

impl Header {
    pub(crate) const SIZE: usize = 14;

    pub(crate) fn new(
        width: u32,
        height: u32,
        channels: ColorChannel,
        color_space: ColorSpace,
    ) -> Self {
        Self {
            width,
            height,
            channels,
            color_space,
        }
    }

    pub(crate) fn try_from_bytes(bytes: impl AsRef<[u8]>) -> Result<Self> {
        let bytes = bytes.as_ref();

        if bytes.len() != Self::SIZE {
            return Err(Error::InvalidHeaderSize(bytes.len()));
        }

        if &bytes[0..4] != QOI_MAGIC {
            return Err(Error::InvalidMagic(bytes[0..4].try_into().unwrap()));
        }

        let width = u32::from_be_bytes(bytes[4..8].try_into().unwrap());
        let height = u32::from_be_bytes(bytes[8..12].try_into().unwrap());
        let channels =
            ColorChannel::from_u8(bytes[12]).ok_or(Error::InvalidChannelNumber(bytes[12]))?;
        let color_space =
            ColorSpace::from_u8(bytes[13]).ok_or(Error::InvalidColorspace(bytes[13]))?;

        Ok(Header {
            width,
            height,
            channels,
            color_space,
        })
    }

    pub(crate) fn as_bytes(&self) -> [u8; Self::SIZE] {
        let mut bytes = [0; Self::SIZE];

        bytes[0..4].copy_from_slice(QOI_MAGIC);
        bytes[4..8].copy_from_slice(&self.width.to_be_bytes());
        bytes[8..12].copy_from_slice(&self.height.to_be_bytes());
        bytes[12] = self.channels as u8;
        bytes[13] = self.color_space as u8;

        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_to_bytes() {
        let header = Header {
            width: 1024,
            height: 512,
            channels: ColorChannel::Rgb,
            color_space: ColorSpace::AllLinear,
        };

        assert_eq!(
            header.as_bytes(),
            [0x71, 0x6f, 0x69, 0x66, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x02, 0x00, 0x03, 0x01]
        );
    }

    #[test]
    fn bytes_to_header() {
        let bytes = [
            0x71, 0x6f, 0x69, 0x66, 0, 0, 0x04, 0x01, 0, 0, 0x02, 0, 4, 1,
        ];

        assert!(matches!(
            Header::try_from_bytes(bytes),
            Ok(Header {
                width: 1025,
                height: 512,
                channels: ColorChannel::Rgba,
                color_space: ColorSpace::AllLinear,
            })
        ));
    }

    #[test]
    fn bytes_to_header_invalid_size() {
        let bytes = [0x71, 0x6f, 0x69, 0x66, 0, 0, 0x04, 0x01, 0];

        assert!(matches!(
            Header::try_from_bytes(bytes),
            Err(Error::InvalidHeaderSize(9))
        ));
    }

    #[test]
    fn bytes_to_header_invalid_magic() {
        let bytes = [
            0x70, 0x6f, 0x69, 0x66, 0, 0, 0x04, 0x01, 0, 0, 0x02, 0, 9, 1,
        ];

        assert!(matches!(
            Header::try_from_bytes(bytes),
            Err(Error::InvalidMagic([0x70, 0x6f, 0x69, 0x66]))
        ));
    }

    #[test]
    fn bytes_to_header_invalid_channels() {
        let bytes = [
            0x71, 0x6f, 0x69, 0x66, 0, 0, 0x04, 0x01, 0, 0, 0x02, 0, 9, 1,
        ];

        assert!(matches!(
            Header::try_from_bytes(bytes),
            Err(Error::InvalidChannelNumber(9))
        ));
    }

    #[test]
    fn bytes_to_header_invalid_colorspace() {
        let bytes = [
            0x71, 0x6f, 0x69, 0x66, 0, 0, 0x04, 0x01, 0, 0, 0x02, 0, 3, 4,
        ];

        assert!(matches!(
            Header::try_from_bytes(bytes),
            Err(Error::InvalidColorspace(4))
        ));
    }
}
