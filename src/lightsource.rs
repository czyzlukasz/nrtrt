use crate::vector::Vector;
use crate::pixel::Color;

#[derive(Clone, Copy, Debug)]
pub struct Lightsource {
    pub position: Vector,
    pub intensity: f64,
    pub color: Color,
}

impl Lightsource{
    pub(crate) fn new(position: &Vector, intensity: f64) -> Lightsource
    {
        Lightsource{
            position: position.clone(),
            intensity,
            color: Color{
                r: 255,
                g: 255,
                b: 255
            }
        }
    }
}