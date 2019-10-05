use crate::ray::Ray;
use crate::vector::Vector;
use crate::shapes::{Collision, MaterialTrait, Shape};
use crate::material::Material;
use crate::pixel::Color;

pub struct Rectangle
{
    pub first_corner: Vector,
    pub dimensions: (f64, f64),
    pub material: Material
}

impl Rectangle
{
    pub fn new(first_corner: &Vector, dimensions: &(f64, f64), material: &Material) -> Rectangle
    {
        Rectangle{
            first_corner: *first_corner,
            dimensions: *dimensions,
            material: *material
        }
        
    }
}

impl MaterialTrait for Rectangle
{
    fn specular_reflectivity(&self) -> f64 {
        self.material.specular_reflectivity
    }

    fn specular_reflection_parameter(&self) -> f64 {
        self.material.specular_reflection_parameter
    }
    fn color(&self) -> Color {
        self.material.color
    }
}

impl Collision for Rectangle
{
    fn can_collide(&self, ray: &Ray) -> bool
    {
        let y_difference = self.first_corner.y - ray.start_position.y;
        if ray.direction.y == 0.
        {
            return false
        }
        let parameter = y_difference / ray.direction.y;
        return parameter > 0.00001;
    }
    fn collision_point(&self, ray: &Ray) -> Option<Vector>
    {
        if self.can_collide(ray) == true
        {
            let y_difference = self.first_corner.y - ray.start_position.y;
            let parameter = y_difference / ray.direction.y;
            let result_position = ray.start_position + ray.direction * parameter;
            // println!("{:?}", result_position);
            return Some(result_position);
        }
        else
        {
            return None;
        }
    }
    fn normal_at_point(&self, _point: &Vector) -> Option<Vector>
    {
        return Some(Vector{
            x: 0.,
            y: -1.,
            z: 0.
        });
    }

    fn up_direction(&self) -> Vector
    {
        return Vector{
            x: 0.,
            y: -1.,
            z: 0.
        };
    }
    fn position(&self) -> Vector
    {
        return self.first_corner;
    }
}

impl Shape for Rectangle
{}


#[cfg(test)]
mod test
{
    use crate::vector::Vector;
    use crate::shapes::Rectangle;
    use crate::shapes::Collision;
    use crate::ray::Ray;
    use crate::material::Material;
    use assert_approx_eq::assert_approx_eq;

    fn get_rectangle() -> Rectangle
    {
        Rectangle::new(&Vector{x: 0., y:0., z:0.,}, &(100., 100.), &Material::default())
    }
    fn get_ray() -> Ray
    {
        let start_position = Vector{
            x: 10.,
            y: 10.,
            z: 10.,
        };
        let direction = Vector{
            x: 2.,
            y: -1.,
            z: 3.,            
        };
        Ray::new(&start_position, &direction)
    }

    #[test]
    fn can_collide() {
        let rect = get_rectangle();
        let mut ray = get_ray();
        assert_eq!(rect.can_collide(&ray), true);
        ray.direction.y = 1.;
        assert_eq!(rect.can_collide(&ray), false);
        ray.start_position.y = -10.;
        assert_eq!(rect.can_collide(&ray), true);
    }

    #[test]
    fn collision_point(){
        let rect = get_rectangle();
        let ray = get_ray();
        let collision_point = rect.collision_point(&ray).unwrap();
        assert_approx_eq!(collision_point.x, 30.);
        assert_approx_eq!(collision_point.y, 0.);
        assert_approx_eq!(collision_point.z, 40.);
    }
}