use std::ops;

#[derive(Clone, Copy, Debug)]
pub struct Color
{
    pub r: u8,  //RED
    pub g: u8,  //GREEN
    pub b: u8,  //BLUE
}

impl Color
{
    //Return the black pixel
    pub fn new() -> Color
    {
        Color {
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

impl ops::Add<Color> for Color
{
    type Output = Color;
    fn add(self, rhs: Color) -> Color
    {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl ops::AddAssign<Color> for Color
{
    fn add_assign(&mut self, rhs: Color)
    {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;

    }
}
#[derive(Clone, Copy, Debug)]
pub struct Pixel
{
    //Color of the pixel
    pub color: Color
}

impl Pixel
{
    pub fn new() -> Pixel
    {
        Pixel{
            color: Color::new()
        }
    }
}
#[cfg(test)]
mod test
{
    use crate::pixel::Color;

    #[test]
    fn to_u32()
    {
        let mut pixel = Color::new();
        assert_eq!(0, pixel.to_u32());

        pixel.r = 23;
        pixel.g = 242;
        pixel.b = 65;
        //--------------------------PADDING--_---R----_---G----_---B----
        let converted_pixel: u32 = 0b00000000_00010111_11110010_01000001;
        assert_eq!(converted_pixel, pixel.to_u32());
    }
}