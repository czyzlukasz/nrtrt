mod vector;
mod pixel;
mod ray;
// mod shape;
mod lightsource;
mod world;
mod camera;
mod shapes;

use vector::Vector;
use lightsource::Lightsource;
use world::World;
use camera::Camera;
use std::rc::Rc;

fn main() {
    println!("Hello, world!");
    let mut world = World::new();
    world.add_shape(Rc::new(shapes::Sphere{radius: 3.,
                                   position: Vector{x: -1.,
                                                    y: 2.,
                                                    z: -13.}}));

    world.add_shape(Rc::new(shapes::Sphere{radius: 2.,
                                   position: Vector{x: 0.7,
                                                    y: 2.,
                                                    z: -11.5}}));

    world.add_light(Rc::new(Lightsource::new(&Vector{x: 0.,
                                                     y: 0.,
                                                     z: 100.},
                    1.0)));

    world.add_light(Rc::new(Lightsource::new(&Vector{x: 10.,
                                                     y: 0.,
                                                     z: -10.},
                    1.5)));

    let mut camera = Camera::new();
    while camera.update()
    {
//        camera.direction.rotate_y(0.5);
        camera.shoot_primary_rays(&world);
        println!("Boop loop");
    }
}
