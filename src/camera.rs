use minifb::{Window, Key, WindowOptions};
use std::rc::Rc;
use crate::{
    shapes::Shape,
    lightsource::Lightsource,
    world::World,
    pixel::{Pixel, Color},
    ray::Ray,
    vector::Vector,
    lambertian::Lambertian,
    raytree::*
};


const FOV: f64 = 70.;
const MAX_RAY_DEPTH: u32 = 2;
const NUM_OF_REFLECTED_RAYS: usize = 20;
//const WIDTH: u32 = 800;
//const HEIGHT: u32 = 600;
const WIDTH: u32 = 200;
const HEIGHT: u32 = 200;

pub struct Camera{
    buffer: Vec<Pixel>,
    pub starting_point: Vector,
    pub direction: Vector,
    lambertian: Lambertian,
    arena: RayArena,
    window: Window
}

impl Camera{
    pub fn new() -> Camera
    {
        Camera{
            buffer: vec![Pixel::new(); (WIDTH * HEIGHT) as usize],
            starting_point: Vector::new(),
            direction: Vector{
                x: 0.,
                y: 0.,
                z: -1.
            },
            lambertian: Lambertian::new(NUM_OF_REFLECTED_RAYS),
            arena: RayArena::new(MAX_RAY_DEPTH),
            window: Window::new("nrtrt", WIDTH as usize, HEIGHT as usize, WindowOptions::default()).unwrap()
        }
    }

    fn get_pixel(&mut self, x: u32, y: u32) -> Option<&mut Pixel>
    {
//        Some(&mut self.buffer[(x + y * WIDTH) as usize])
        self.buffer.get_mut((x + y * WIDTH) as usize)
    }

    pub fn update(&mut self) -> bool
    {
        //TODO: change that!
        let mut temp_buffer: Vec<u32> = Vec::with_capacity((WIDTH * HEIGHT) as usize);
        for pixel in 0..(WIDTH * HEIGHT)
        {
            temp_buffer.push(self.buffer[pixel as usize].color.to_u32());
        }
        self.window.update_with_buffer(&temp_buffer).unwrap();
        //Clear the buffer
        self.buffer = vec![Pixel::new(); (WIDTH * HEIGHT) as usize];
        println!("Debug, num of rays: {}", self.arena.nodes.len());
        return self.window.is_open() && !self.window.is_key_down(Key::Escape);
    }
    pub fn shoot_primary_rays(&mut self, world: &World)
    {
        let pixel_to_pixel_angle = FOV / WIDTH as f64;
        let first_pixel_angle_horizontal = (WIDTH as i32 / -2) as f64 * pixel_to_pixel_angle;
        let first_pixel_angle_vertical = (HEIGHT as i32 / -2) as f64 * pixel_to_pixel_angle;
        //Clear the arena
        self.arena.nodes.clear();

        for x in 0..WIDTH
        {
            for y in 0..HEIGHT
            {
                let mut ray_direction = self.direction;
                ray_direction.rotate_y(first_pixel_angle_horizontal + pixel_to_pixel_angle * x as f64);   //Rotate ray horizontally
                ray_direction.rotate_x(first_pixel_angle_vertical + pixel_to_pixel_angle * y as f64);   //Rotate ray vertically


                let ray = Ray::new(&self.starting_point, &ray_direction);
                let items = world.item_that_collide(&ray);
                if let Some(item) = items
                {
                    let collision_point = item.collision_point(&ray).unwrap();
                    let normal = item.normal_at_point(&collision_point).unwrap();

                    // Create reflected rays and add them to the arena
                    let node_id = self.arena.add_node(NodeId::Root, &Ray::new(&collision_point, &normal));
                    self.shoot_reflected_rays(world, &self.lambertian.get_offsets().clone(), node_id);
                    let color = self.calculate_color(node_id);
                }
                else
                {
                    self.get_pixel(x, y).unwrap().color = Color{r: 128, g: 218, b: 235};
                }
            }
        }
    }

    fn shoot_reflected_rays(&mut self, world: &World, offsets: &Vec<Vector>, id: NodeId){
        let ray_node_opt = self.arena.get_node(id);
        //If parent exists
        if let Some(ray_node) = ray_node_opt{
            if ray_node.recursion_depth >= MAX_RAY_DEPTH{
                return;
            }
            //If the collision occurred
            if let Some(collision_shape) = world.item_that_collide(&ray_node.ray){
                //Iterate over all possible reflected rays and add it only if they collide
                let new_collision_point = collision_shape.collision_point(&ray_node.ray).unwrap();
                for offset in offsets.iter(){
                    let new_direction = collision_shape.normal_at_point(&new_collision_point).unwrap() + *offset;
                    let new_ray = Ray::new(&new_collision_point, &new_direction);
                    let new_node_id = self.arena.add_node(id, &new_ray);
                    self.shoot_reflected_rays(world, offsets, new_node_id);
                }
            }
        }
    }

    fn calculate_color(&self, id: NodeId) -> Color{
        if let NodeId::Parent(parent_id) = id{
            let end_nodes = self.arena.get_last_nodes(id);

        }
        Color::white()
    }

    fn can_light_reach_point(&self, world: &World, light_position:&Vector, point: &Vector) -> bool{
        let ray = Ray::new(point, &(*light_position - *point));
        if let Some(_) = world.item_that_collide(&ray){
            return true;
        }
        false
    }
}