use crate::pixel::Color;

#[derive(Copy, Clone)]
pub struct Material
{
    pub reflectivity: f64,
    pub color: Color
}

impl Material
{
    #[warn(dead_code)]
    pub fn new(reflectivity: f64, color: Color) -> Material
    {
        Material{
            reflectivity,
            color
        }
    }

    #[warn(dead_code)]
    pub fn default() -> Material
    {
        Material{
            reflectivity: 0.8,
            color: Color::white()
        }
    }

    #[warn(dead_code)]
    pub fn new_color(r: u8, g: u8, b: u8) -> Material
    {
        Material{
            reflectivity: 0.8,
            color: Color{
                r,
                g,
                b
            }
        }
    }
    
    pub fn new_color_ref(r: u8, g: u8, b: u8, refl: f64) -> Material
    {
        Material{
            reflectivity: refl,
            color: Color{
                r,
                g,
                b
            }
        }
    }
}