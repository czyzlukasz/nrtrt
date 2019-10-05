use crate::vector::Vector;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct Ray
{
    pub start_position: Vector,
    pub direction: Vector,
    //ID-type tree was chosen because it will later be easier to split the ray generation
    //To multiple threads
    id: u32,
    parent: u32,
    childrens: Vec<u32>
}

impl Ray
{
    pub fn new(start_position: &Vector, direction: &Vector) -> Ray
    {
        Ray{
            start_position: *start_position,
            direction: *direction,
            id: 0,
            parent: 0,
            childrens: Vec::new()
        }
    }
    //Calculate the closest distance of the point and the ray (it's 0 if ray comes trough the point)
    pub fn distance_to_point(&self, point: Vector) -> f64
    {
        let start_to_point = point - self.start_position;
        return (self.direction * start_to_point).distance() / self.direction.distance();
    }
}


#[cfg(test)]
mod test
{
    use crate::vector::Vector;
    use crate::ray::Ray;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn distance_to_point()
    {
        let start_position = Vector{
            x: 1.,
            y: 2.,
            z: 3.
        };
        let direction = Vector{
            x: -1.,
            y: -0.5,
            z: 7.,
        };
        let ray = Ray::new(&start_position, &direction);

        let point = Vector{
            x: 5.,
            y: 4.,
            z: 7.
        };
        let distance = ray.distance_to_point(point);
        assert_approx_eq!(distance, 5.047042);
    }
}