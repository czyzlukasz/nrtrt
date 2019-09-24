use crate::ray::Ray;
use crate::vector::Vector;
use std::fmt::Error;

pub trait Collision
{
    fn can_collide(&self, ray: Ray) -> bool;
    fn collision_point(&self, ray: Ray) -> Result<Vector, Error>;
    fn normal_at_point(&self, point: Vector) -> Vector;

}

pub trait Position
{
    fn up_direction(&self) -> Vector;
    fn position(&self) -> Vector;
}

struct Sphere
{
    radius: f64,
    position: Vector
}

impl Position for Sphere
{
    fn up_direction(&self) -> Vector
    {
        Vector{
            x: 0.,
            y: 0.,
            z: 1.
        }
    }
    fn position(&self) -> Vector
    {
        return self.position;
    }
}

impl Collision for Sphere
{
    fn can_collide(&self, ray: Ray) -> bool
    {
        return ray.distance_to_point(self.position) <= self.radius;
    }
    fn collision_point(&self, ray: Ray) -> Result<Vector, Error>
    {
        return Err(Error);
    }
    fn normal_at_point(&self, point: Vector) -> Vector
    {
        return Vector{
            x: 1.,
            y: 1.,
            z: 1.
        }
    }
}

#[cfg(test)]
mod test
{
    use crate::vector::Vector;
    use crate::shapes::Sphere;
    use assert_approx_eq::assert_approx_eq;

//    #[test]
//    fn can_collide()
//    {
//    }
}