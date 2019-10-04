use crate::pixel::Color;

#[derive(Copy, Clone)]
pub struct Material
{
    pub specular_reflectivity: f64,
    pub specular_reflection_parameter: f64,
    pub color: Color
}

impl Material
{
    pub fn new(specular_reflectivity: f64, specular_reflection_parameter: f64, color: Color) -> Material
    {
        Material{
            specular_reflectivity,
            specular_reflection_parameter,
            color
        }
    }

    pub fn default() -> Material
    {
        Material{
            specular_reflectivity: 0.6,
            specular_reflection_parameter: 4.,
            color: Color::white()
        }
    }
    pub fn new_color(r: u8, g: u8, b: u8) -> Material
    {
        Material{
            specular_reflectivity: 0.6,
            specular_reflection_parameter: 4.,
            color: Color{
                r,
                g,
                b
            }
        }
    }
}