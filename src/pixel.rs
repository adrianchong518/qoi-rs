pub struct Pixel<const N: usize>([u8; N]);

impl Pixel<3> {
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self([r, g, b])
    }
}

impl Pixel<4> {
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self([r, g, b, a])
    }
}

impl<const N: usize> Pixel<N>
where
    Pixel<N>: SupportedChannels,
{
    pub(crate) fn index_hash(&self) -> usize {
        (self.0[0] as usize * 3
            + self.0[1] as usize * 5
            + self.0[2] as usize * 7
            + match N {
                3 => 255 * 11,
                4 => self.0[3] as usize + 11,
                _ => unreachable!(),
            })
            % 64
    }
}

pub trait SupportedChannels {}

impl SupportedChannels for Pixel<3> {}
impl SupportedChannels for Pixel<4> {}
