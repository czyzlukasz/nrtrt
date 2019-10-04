
use crate::ray::Ray;
use crate::vector::Vector;
use crate::shapes::{Collision, Material, Shape};

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
        if t < -0.000000000001  //To avoid f64 precision error, because it could still collide and be very small negative number
        {
            None
        }
        else
        {
            Some(ray.start_position + ray.direction * t)
        }
    }
    fn normal_at_point(&self, point: Vector) -> Option<Vector>
    {
        let distance = (self.position - point).distance();
        if distance - self.radius > 0.0001
        {
            println!("Difference in length: {} != {}", distance, self.radius);
            None
        }
        else
        {
            Some(point - self.position)
        }
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

impl Material for Sphere
{
    fn specular_reflectivity(&self) -> f64 {
        0.6
    }

    fn specular_reflection_parameter(&self) -> f64 {
        4.
    }
}
impl Shape for Sphere
{
}

#[cfg(test)]
mod test
{
    use crate::vector::Vector;
    use crate::shapes::Sphere;
    use crate::shapes::Collision;
    use crate::ray::Ray;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn can_collide()
    {
        let ray = Ray{
            start_position: Vector{
                x: -1.,
                y: 0.,
                z: -1.},
            direction: Vector{
                x: 1.,
                y: 1.,
                z: 1.}
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
    fn can_collide_inside_on_border()
    {
        let ray = Ray{
            start_position: Vector{
                x:1.,
                y:0.,
                z:0.
            },
            direction: Vector{
                x:-1.,
                y:0.,
                z:0.
            }
        };
        let sphere_on_border = Sphere{
            radius: 1.,
            position: Vector{
                x: 0.,
                y: 0.,
                z: 0.
            }
        };
        let sphere_inside = Sphere{
            radius: 2.,
            position: Vector{
                x: 0.,
                y: 0.,
                z: 0.
            }
        };
        assert_eq!(true, sphere_on_border.can_collide(ray));
        assert_eq!(true, sphere_inside.can_collide(ray));
    }

    #[test]
    fn collision_point()
    {
        let ray = Ray{
            start_position: Vector{
            x: -1.,
            y: 1.,
            z: 1.},
            direction: Vector{
                x: 1.,
                y: 0.,
                z: 0.}
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
            None => panic!()
        };
    }

    #[test]
    fn normal_at_point()
    {
        let sphere = Sphere{
            radius: 1.,
            position: Vector{
                x: 0.,
                y: 0.,
                z: 0.
            }
        };

        let point = Vector{
            x: 0.70710,
            y: 0.70710,
            z: 0.
        };
        let normal = sphere.normal_at_point(point).unwrap();
        //Add '==' to vector
        //Because the shape is a sphere and its located in (0,0,0), the normal
        //Vector is the same as the point vector.
        assert_eq!(normal.x, point.x);
        assert_eq!(normal.y, point.y);
        assert_eq!(normal.z, point.z);
    }
}