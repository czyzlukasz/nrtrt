use crate::shapes::{Shape};
use crate::ray::Ray;
use crate::lightsource::Lightsource;
use std::rc::Rc;
use std::f64;

pub struct World{
    shapes: Vec<Rc<dyn Shape>>,
    pub lights: Vec<Rc<Lightsource>>
}

impl World{
    pub fn new() -> World
    {
        World{
            shapes: vec![],
            lights: vec![]
        }
    }

    pub fn add_shape(& mut self, shape: Rc<dyn Shape>)
    {
        self.shapes.push( shape);
    }

    pub fn add_light(&mut self, light: Rc<Lightsource>)
    {
        self.lights.push(light);
    }
    pub fn item_that_collide(&self, ray: Ray) -> Option<Rc<dyn Shape>>
    {
        let mut closest_item_that_collide: Option<Rc<dyn Shape>> = None;
        let mut smallest_distance = f64::MAX;
        for shape in self.shapes.iter()
        {
            // Check if ray will even collide with shape to avoid unnecessary calculations
            if shape.can_collide(ray)
            {
                let point = shape.collision_point(ray);
                if let Some(collision_point) = point
                {
                    // Calculate the distance to closest collision, because ray will end in
                    // the first collision
                    let distance = (collision_point - ray.start_position).distance();
                    if distance < smallest_distance
                    {
                        smallest_distance = distance;
                        closest_item_that_collide = Some(Rc::clone(&shape));
                    }
                }
            }
        }
        match closest_item_that_collide
        {
            Some(item) => Some(item),
            None => None
        }
    }
}