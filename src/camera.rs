use crate::ray::Ray;
use crate::vector::Vector;

const FOV: f64 = 70.;
const WIDTH: u32 = 100;
const HEIGHT: u32 = 100;

pub struct Camera{
    buffer: Vec<u32>
}

impl Camera{
    fn new() -> Camera
    {
        Camera{
            buffer: vec![0; (WIDTH * HEIGHT) as usize]
        }
    }
}