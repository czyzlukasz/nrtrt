use crate::vector::Vector;
use crate::pixel::Color;

#[derive(Clone, Copy, Debug)]
pub struct Lightsource {
    position: Vector,
    intensity: f64,
    color: Color,
}

impl Lightsource{
    pub(crate) fn new(position: &Vector) -> Lightsource
    {
        Lightsource{
            position: position.clone(),
            intensity: 1.,
            color: Color{
                r: 255,
                g: 255,
                b: 255
            }
        }
    }
}