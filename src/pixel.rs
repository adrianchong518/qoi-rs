#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

impl From<Pixel<3>> for Pixel<4> {
    fn from(pixel: Pixel<3>) -> Self {
        Self(pixel.as_inner_rgba())
    }
}

pub trait SupportedChannels {
    fn new_initial() -> Self;

    fn red(&self) -> u8;
    fn green(&self) -> u8;
    fn blue(&self) -> u8;
    fn alpha(&self) -> u8;

    fn as_inner_rgb(&self) -> [u8; 3];
    fn as_inner_rgba(&self) -> [u8; 4];

    fn as_rgba(&self) -> Pixel<4>;
}

impl SupportedChannels for Pixel<3> {
    fn new_initial() -> Self {
        Self([0, 0, 0])
    }

    fn red(&self) -> u8 {
        self.0[0]
    }

    fn green(&self) -> u8 {
        self.0[1]
    }

    fn blue(&self) -> u8 {
        self.0[2]
    }

    fn alpha(&self) -> u8 {
        255
    }

    fn as_inner_rgb(&self) -> [u8; 3] {
        self.0
    }

    fn as_inner_rgba(&self) -> [u8; 4] {
        [self.red(), self.green(), self.blue(), self.alpha()]
    }

    fn as_rgba(&self) -> Pixel<4> {
        (*self).into()
    }
}

impl SupportedChannels for Pixel<4> {
    fn new_initial() -> Self {
        Self([0, 0, 0, 255])
    }

    fn red(&self) -> u8 {
        self.0[0]
    }

    fn green(&self) -> u8 {
        self.0[1]
    }

    fn blue(&self) -> u8 {
        self.0[2]
    }

    fn alpha(&self) -> u8 {
        self.0[3]
    }

    fn as_inner_rgb(&self) -> [u8; 3] {
        self.0.split_at(3).0.try_into().unwrap()
    }

    fn as_inner_rgba(&self) -> [u8; 4] {
        self.0
    }

    fn as_rgba(&self) -> Pixel<4> {
        *self
    }
}

impl<const N: usize> Pixel<N>
where
    Pixel<N>: SupportedChannels,
{
    pub(crate) fn index_hash(&self) -> usize {
        (self.red() as usize * 3
            + self.green() as usize * 5
            + self.blue() as usize * 7
            + self.alpha() as usize * 11)
            % 64
    }
}

impl<const N: usize> Default for Pixel<N>
where
    Pixel<N>: SupportedChannels,
{
    fn default() -> Self {
        Self([0; N])
    }
}
