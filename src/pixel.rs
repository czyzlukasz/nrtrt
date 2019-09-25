#[derive(Clone, Copy, Debug)]
pub struct Pixel
{
    pub r: u8,  //RED
    pub g: u8,  //GREEN
    pub b: u8,  //BLUE
}

impl Pixel
{
    pub fn new() -> Pixel
    {
        Pixel{
            r: 0,
            g: 0,
            b: 0,
        }
    }

    pub fn to_u32(&self) -> u32
    {
        let r = self.r as u32 * 256 * 256;
        let g = self.g as u32 * 256;
        let b = self.b as u32;
        r + g + b
    }
}
