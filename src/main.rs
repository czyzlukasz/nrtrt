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
use camera::{Camera, UpdateStatus};
use material::Material;
use std::rc::Rc;

fn main() {
    println!("Hello, world!");
    let mut world = World::new();
    world.add_shape(Rc::new(shapes::Sphere{radius: 3.5,
                                           position: Vector{x: -3.,
                                                            y: 1.5,
                                                            z: -12.},
                                           material: Material::new_color_ref(50, 255, 10, 0.6, false)}));

    world.add_shape(Rc::new(shapes::Sphere{radius: 2.,
                                           position: Vector{x: -1.2,
                                                            y: -4.,
                                                            z: -8.5},
                                           material: Material::new_color_ref(150, 80, 220, 0.95, true)}));

    world.add_shape(Rc::new(shapes::Sphere{radius: 2.5,
                                           position: Vector{x: 3.2,
                                                            y: 2.,
                                                            z: -9.},
                                           material: Material::new_color_ref(220, 220, 20, 0.9, true)}));

    world.add_shape(Rc::new(shapes::Sphere{radius: 1.5,
                                           position: Vector{x: 3.2,
                                                            y: -2.,
                                                            z: -9.},
                                           material: Material::new_color_ref(230, 5, 10, 0.9, false)}));

    world.add_shape(Rc::new(shapes::Rectangle::new(&Vector{x: 0.,
                                                         y: 3.8,
                                                         z: 0.},
                                                  &(20., 20.,),
                                                  &Material::new_color_ref(155, 105, 40, 1., true))));

    // world.add_light(Rc::new(Lightsource::new(&Vector{x: 30.,
    //                                                  y: 0.,
    //                                                  z: -10.},
    //                 0.2)));

    // world.add_light(Rc::new(Lightsource::new(&Vector{x: 10000.,
    //                                                  y: 0.,
    //                                                  z: 0.},
    //                 0.3)));

    // world.add_light(Rc::new(Lightsource::new(&Vector{x: -10000.,
    //                                                  y: -10000.,
    //                                                  z: 0.},
    //                 0.6)));

    world.add_light(Rc::new(Lightsource::new(&Vector{x: 10000.,
        y: -10000.,
        z: 10000.},
                                             0.4)));

    world.add_light(Rc::new(Lightsource::new(&Vector{x: 10000.,
        y: -1000.,
        z: 10000.},
                                             0.4)));

    let mut camera = Camera::new();
    let mut status = UpdateStatus::NotFinished;
    let mut saved = false;
    while status != UpdateStatus::AboutToExit
    {
        status = camera.update();
        match status{
            UpdateStatus::NotFinished =>{
                camera.shoot_primary_rays(&world);
                println!("Total ray shot count: {}", camera.total_num_of_rays);
            },
            UpdateStatus::Finished =>{
                if saved == false{
                    saved = true;
                    camera.save_image();
                }
            },
            UpdateStatus::AboutToExit => break
        }
    }
}
