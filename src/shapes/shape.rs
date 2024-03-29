use crate::ray::Ray;
use crate::vector::Vector;
use crate::pixel::Color;

pub trait Collision
{
    //Checks if ray can hit the shape's body
    fn can_collide(&self, ray: &Ray) -> bool;
    //Returns (if possible) the first point of intersection of shape with given ray
    fn collision_point(&self, ray: &Ray) -> Option<Vector>;
    //Returns (if possible) the normal vector of the shape for a given point on the shape
    fn normal_at_point(&self, point: &Vector) -> Option<Vector>;
    // Returns the "up" direction of shape
    fn up_direction(&self) -> Vector;
    // Returns the center of the shape
    fn position(&self) -> Vector;
}

pub trait MaterialTrait
{
    fn reflectivity(&self) -> f64;
    fn color(&self) -> Color;
    fn is_specular(&self) -> bool;
}

pub trait Shape: Collision + MaterialTrait
{}
