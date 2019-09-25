use std::rc::Rc;

mod vector;
mod ray;
mod shapes;
mod world;
mod camera;
mod pixel;

use vector::Vector;
use ray::Ray;
use shapes::Sphere;
use world::World;
use camera::Camera;

fn main() {
    println!("Hello, world!");
    let mut world = World::new();
    world.add_shape(Rc::new(Sphere{radius: 1.,
                                                position: Vector{x: 0.,
                                                                 y: 0.,
                                                                 z: -10.}}));
    world.add_shape(Rc::new(Sphere{radius: 2.,
                                                position: Vector{x: 3.,
                                                                 y: 2.,
                                                                 z: -13.}}));

    let mut camera = Camera::new();
    camera.shoot_rays(&world);
    while camera.update()
    {

    }
}
