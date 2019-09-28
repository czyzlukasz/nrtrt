use crate::vector::Vector;
use crate::ray::Ray;
use crate::pixel::{Pixel, Color};
use crate::world::World;
use minifb::{Window, Key, WindowOptions};

const FOV: f64 = 70.;
const WIDTH: u32 = 480;
const HEIGHT: u32 = 320;

pub struct Camera{
    buffer: Vec<Pixel>,
    pub starting_point: Vector,
    pub direction: Vector,
    window: Window
}

impl Camera{
    pub fn new() -> Camera
    {
        Camera{
            buffer: vec![Pixel::new(); (WIDTH * HEIGHT) as usize],
            starting_point: Vector::new(),
            direction: Vector{
                x: 0.,
                y: 0.,
                z: -1.
            },
            window: Window::new("nrtrt", WIDTH as usize, HEIGHT as usize, WindowOptions::default()).unwrap()
        }
    }

    fn get_pixel(&mut self, x: u32, y: u32) -> Option<&mut Pixel>
    {
//        Some(&mut self.buffer[(x + y * WIDTH) as usize])
        self.buffer.get_mut((x + y * WIDTH) as usize)
    }

    pub fn update(&mut self) -> bool
    {
        //TODO: change that!
        let mut temp_buffer: Vec<u32> = Vec::new();
        for pixel in 0..(WIDTH * HEIGHT)
        {
            temp_buffer.push(self.buffer[pixel as usize].color.to_u32());
        }
        self.window.update_with_buffer(&temp_buffer).unwrap();
        //Clear the buffer
        self.buffer = vec![Pixel::new(); (WIDTH * HEIGHT) as usize];
        return self.window.is_open() && !self.window.is_key_down(Key::Escape);
    }
    pub fn shoot_primary_rays(&mut self, world: &World)
    {
        let pixel_to_pixel_angle = FOV / WIDTH as f64;
        let first_pixel_angle_horizontal = (WIDTH as i32 / -2) as f64 * pixel_to_pixel_angle;
        let first_pixel_angle_vertical = (HEIGHT as i32 / -2) as f64 * pixel_to_pixel_angle;
        for x in 0..WIDTH
        {
            for y in 0..HEIGHT
            {
                let mut ray_direction = self.direction;
                ray_direction.rotate_y(first_pixel_angle_horizontal + pixel_to_pixel_angle * x as f64);   //Rotate ray horizontally
                ray_direction.rotate_x(first_pixel_angle_vertical + pixel_to_pixel_angle * y as f64);   //Rotate ray vertically
                let ray = Ray{
                    direction: ray_direction,
                    start_position: self.starting_point
                };
                let items = world.item_that_collide(ray);
                if let Some(item) = items
                {
                    //TODO remove that
                    let collision_point = item.collision_point(ray).unwrap();
                    let normal = item.normal_at_point(collision_point).unwrap();
                    let reflection = ray_direction.reflection(normal);
                    let mut dot_product = - (ray.direction.normalized()).dot(reflection.normalized());
                    if dot_product < 0.
                    {
                        dot_product = 0.;
                    }
                    else
                    {
                        dot_product = dot_product.powf(0.7);
                    }
                    let pixel_value = (dot_product * 255.) as u8;
                    let mut pixel = &mut self.get_pixel(x, y).unwrap().color;
                    pixel.r = pixel_value;
                    pixel.g = pixel_value;
                    pixel.b = pixel_value;
//                    println!("{:?}",(dot_product, pixel_value, pixel));
                }
            }
        }
    }

    fn calculate_light_intensity(&self, ray: &Ray, collision_point: &Vector) -> Color
    {
        Color::new()
    }
}