use crate::ray::Ray;
use crate::vector::Vector;

pub trait Collision
{
    fn can_collide(&self, ray: Ray) -> bool;
    fn collision_point(&self, ray: Ray) -> Option<Vector>;
    fn normal_at_point(&self, point: Vector) -> Vector;
    fn up_direction(&self) -> Vector;
    fn position(&self) -> Vector;
}

pub struct Sphere
{
    pub radius: f64,
    pub position: Vector
}

impl Collision for Sphere
{
    fn can_collide(&self, ray: Ray) -> bool
    {
        return ray.distance_to_point(self.position) <= self.radius;
    }
    fn collision_point(&self, ray: Ray) -> Option<Vector>
    {
        // start position to center
        let oc = ray.start_position - self.position;
        // Parameters a, b, c used in equation for solving t
        let a = ray.direction.dot(ray.direction);
        let b = 2. * oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let delta = b * b - 4. * a * c;
        // delta has to be positive
        if delta < 0.
        {
            return None
        }
        // t is a factor which describes the point of interception
        // point of collision = start of the ray + t * direction of the ray
        let t = (-b - delta.sqrt()) / (2. * a);
        if t < 0.
        {
            None
        }
        else
        {
//            println!("{:?}, {:?}", ray.start_position + ray.direction * t, (a,b,c,delta,t));
            Some(ray.start_position + ray.direction * t)
        }
    }
    fn normal_at_point(&self, point: Vector) -> Vector
    {
        point
//        return Vector{
//            x: 1.,
//            y: 1.,
//            z: 1.
//        }
    }
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

#[cfg(test)]
mod test
{
    use crate::vector::Vector;
    use crate::shapes::{Sphere, Collision};
    use assert_approx_eq::assert_approx_eq;
    use crate::ray::Ray;

    #[test]
    fn can_collide()
    {
        let start_position = Vector{
            x: 1.,
            y: 0.,
            z: 1.
        };
        let direction = Vector{
            x: 1.,
            y: 1.,
            z: 1.,
        };
        let ray = Ray{
            start_position,
            direction
        };

        let position = Vector{
            x: 0.,
            y: 0.,
            z: 0.
        };
        let sphere_small = Sphere{
            radius: 0.5,
            position
        };
        let sphere_just_big_enough = Sphere{
            radius: 1.0,
            position
        };
        let sphere_huge = Sphere{
            radius: 5.,
            position
        };

        assert_eq!(true, sphere_just_big_enough.can_collide(ray));
        assert_eq!(true, sphere_huge.can_collide(ray));
        assert_eq!(false, sphere_small.can_collide(ray));
    }

    #[test]
    fn collision_point()
    {
        let start_position = Vector{
            x: -1.,
            y: 1.,
            z: 1.
        };
        let direction = Vector{
            x: 1.,
            y: 0.,
            z: 0.,
        };
        let ray = Ray{
            start_position,
            direction
        };

        let sphere = Sphere{
            radius: 2.,
            position: Vector{
                x: 10.,
                y: 0.,
                z: 0.
            }
        };
        let result = sphere.collision_point(ray);
        match result {
            Some(point) => {
                assert_approx_eq!(point.x, 8.585786);
                assert_approx_eq!(point.y, 1.);
                assert_approx_eq!(point.z, 1.);
            },
            None => assert!(false)
        };
    }
}