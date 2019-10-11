mod vector;
mod pixel;
mod ray;
// mod shape;
mod lightsource;
mod world;
mod camera;
mod shapes;
mod material;
mod lambertian;
mod raytree;

use vector::Vector;
use lightsource::Lightsource;
use world::World;
use camera::Camera;
use material::Material;
use std::rc::Rc;

fn main() {
    println!("Hello, world!");
    let mut world = World::new();
    world.add_shape(Rc::new(shapes::Sphere{radius: 3.,
                                           position: Vector{x: -1.,
                                                            y: 2.,
                                                            z: -13.},
                                           material: Material::new_color_ref(255, 255, 255, 0.95)}));

    world.add_shape(Rc::new(shapes::Sphere{radius: 2.,
                                           position: Vector{x: 0.7,
                                                            y: 2.,
                                                            z: -11.5},
                                           material: Material::new_color_ref(150, 80, 220, 0.95)}));

    world.add_shape(Rc::new(shapes::Rectangle::new(&Vector{x: 0.,
                                                         y: 3.8,
                                                         z: 0.},
                                                  &(20., 20.,),
                                                  &Material::new_color_ref(50, 255, 10, 0.3))));

    // world.add_light(Rc::new(Lightsource::new(&Vector{x: 30.,
    //                                                  y: 0.,
    //                                                  z: -10.},
    //                 0.2)));

    // world.add_light(Rc::new(Lightsource::new(&Vector{x: 0.,
    //                                                  y: -5.,
    //                                                  z: -50.},
    //                 0.3)));

    world.add_light(Rc::new(Lightsource::new(&Vector{x: -10.,
                                                     y: -10.,
                                                     z: -10.},
                    0.6)));

    world.add_light(Rc::new(Lightsource::new(&Vector{x: -0.,
        y: -5.,
        z: -0.},
                                             0.2)));

    let mut camera = Camera::new();
    while camera.update()
    {
//        camera.direction.rotate_y(0.5);
        camera.shoot_primary_rays(&world);
        println!("Boop loop");
    }
}
