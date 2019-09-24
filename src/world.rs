use crate::shapes::{Collision, Position};
use crate::ray::Ray;
use std::rc::Rc;
use std::ops::Deref;
use std::fmt::Error;

//Combined trait
trait CollisionPosition: Collision + Position{}

pub struct World{
    shapes: Vec<Rc<dyn CollisionPosition>>
}

impl World{
    fn new() -> World
    {
        World{
            shapes: vec![]
        }
    }

    fn add_shape(& mut self, shape: Rc<dyn CollisionPosition>)
    {
        self.shapes.push( shape);
    }

    fn items_that_collide(&self, ray: Ray) -> Result<Rc<dyn CollisionPosition>, Error>
    {
        let mut closest_item_that_collide: Option<Rc<dyn CollisionPosition>> = None;
        let mut smallest_distance = -1.;
        for shape in self.shapes.iter()
        {
            // Check if ray will even collide with shape to avoid unnecessary calculations
            if shape.can_collide(ray)
            {
                let point = shape.collision_point(ray);
                match point{
                    Ok(collision_point) => {
                        // Calculate the distance to closest collistion, because ray will end in
                        // the first collision
                        let distance = (collision_point - shape.position()).distance();
                        if distance < smallest_distance
                        {
                            smallest_distance = distance;
                            closest_item_that_collide = Some(shape.clone());
                        }
                    }
                    Err(_) => continue
                };
            }
        }
        match closest_item_that_collide
        {
            Some(item) => Ok(item),
            None => Err(Error)
        }
    }
}