use crate::pixel::Color;

#[derive(Copy, Clone)]
pub struct Material
{
    pub reflectivity: f64,
    pub color: Color,
    pub is_specular: bool
}

impl Material
{
    #[allow(dead_code)]
    pub fn new(reflectivity: f64, color: Color, is_specular: bool) -> Material
    {
        Material{
            reflectivity,
            color,
            is_specular
        }
    }

    #[allow(dead_code)]
    pub fn default() -> Material
    {
        Material{
            reflectivity: 0.8,
            color: Color::white(),
            is_specular: true
        }
    }

    #[allow(dead_code)]
    pub fn new_color(r: u8, g: u8, b: u8) -> Material
    {
        Material{
            reflectivity: 0.8,
            color: Color{
                r,
                g,
                b
            },
            is_specular: true
        }
    }

    pub fn new_color_ref(r: u8, g: u8, b: u8, refl: f64, is_specular: bool) -> Material
    {
        Material{
            reflectivity: refl,
            color: Color{
                r,
                g,
                b
            },
            is_specular
        }
    }
}