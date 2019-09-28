use std::rc::Rc;

mod vector;
mod ray;
mod shapes;
mod world;
mod camera;
mod pixel;
mod lightsource;

use vector::Vector;
use shapes::Sphere;
use world::World;
use camera::Camera;
use lightsource::Lightsource;

fn main() {
    println!("Hello, world!");
    let mut world = World::new();
    world.add_shape(Rc::new(Sphere{radius: 3.,
                                                position: Vector{x: -1.,
                                                                 y: 2.,
                                                                 z: -13.}}));
    world.add_shape(Rc::new(Sphere{radius: 2.,
                                                position: Vector{x: 0.7,
                                                                 y: 2.,
                                                                 z: -11.5}}));
    world.add_light(Rc::new(Lightsource::new(&Vector{x: 4.,
                                                                          y: 10.,
                                                                          z: - 14.})));

    let mut camera = Camera::new();
    while camera.update()
    {
//        camera.direction.rotate_y(0.5);
        camera.shoot_primary_rays(&world);
    }
}
