pub(crate) const QOI_MAGIC: &[u8; 4] = b"qoif";
pub(crate) const QOI_END_MARKER: &[u8; 8] = b"\x00\x00\x00\x00\x00\x00\x00\x01";

pub(crate) const QOI_OP_RGB: u8 = 0b1111_1110;
pub(crate) const QOI_OP_RGBA: u8 = 0b1111_1111;

pub(crate) const QOI_OP_INDEX: u8 = 0b0000_0000;
pub(crate) const QOI_OP_DIFF: u8 = 0b0100_0000;
pub(crate) const QOI_OP_LUMA: u8 = 0b1000_0000;
pub(crate) const QOI_OP_RUN: u8 = 0b1100_0000;
