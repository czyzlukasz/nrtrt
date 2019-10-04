use crate::vector::Vector;
use crate::ray::Ray;
use crate::pixel::{Pixel, Color};
use crate::world::World;
use minifb::{Window, Key, WindowOptions};
use crate::lightsource::Lightsource;
use crate::shapes::Shape;
use std::rc::Rc;

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
        let mut temp_buffer: Vec<u32> = Vec::with_capacity((WIDTH * HEIGHT) as usize);
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
                    let collision_point = item.collision_point(ray).unwrap();
                    let normal = item.normal_at_point(collision_point).unwrap();
                    for light in world.lights.iter()
                    {
                        if self.can_reflected_ray_hit_light(world, &collision_point, &light)
                        {
                            let color = self.calculate_light_intensity(&ray, &item, &collision_point,&normal, &light);
//                            self.get_pixel(x, y).unwrap().color = color;
                            self.get_pixel(x, y).unwrap().color += color;
                            // println!("{:?}", color);
                        }
                    }
                }
                else
                {
                    self.get_pixel(x, y).unwrap().color = Color{r: 128, g: 218, b: 235};
                }
            }
        }
    }

    fn can_reflected_ray_hit_light(&self, world: &World, collision_point: &Vector, light: &Lightsource) -> bool
    {
        let ray = Ray{
            start_position: *collision_point,
            direction: light.position - *collision_point,

        };
        match world.item_that_collide(ray){
            Some(_) => false,
            None => true
        }
    }

    fn calculate_light_intensity(&self, ray: &Ray, material: &Rc<dyn Shape>, collision_point: &Vector, normal: &Vector, light: &Lightsource) -> Color
    {
        let vector_from_collision_to_light = light.position - *collision_point;
        let angle = ray.direction.normalized().reflection(normal.normalized()).dot(vector_from_collision_to_light.normalized());
        if angle > 0.// && angle < 0.5
        {
            let intensity = material.specular_reflectivity() * light.intensity * angle.powf(material.specular_reflection_parameter());
            let normalized_intensity = intensity.min(1.);
            material.color() * normalized_intensity
        }
        else {
            Color{
                r: 0,
                g: 0,
                b: 0
            }
        }

    }
}
