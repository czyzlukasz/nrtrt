#[derive(Clone, Copy, Debug)]
pub struct Pixel
{
    pub r: u8,  //RED
    pub g: u8,  //GREEN
    pub b: u8,  //BLUE
}

impl Pixel
{
    //Return the black pixel
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

#[cfg(test)]
mod test
{
    use crate::pixel::Pixel;

    #[test]
    fn to_u32()
    {
        let mut pixel = Pixel::new();
        assert_eq!(0, pixel.to_u32());

        pixel.r = 23;
        pixel.g = 242;
        pixel.b = 65;
        //--------------------------PADDING--_---R----_---G----_---B----
        let converted_pixel: u32 = 0b00000000_00010111_11110010_01000001;
        assert_eq!(converted_pixel, pixel.to_u32());
    }
}