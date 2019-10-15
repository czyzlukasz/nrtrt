use minifb::{Window, Key, WindowOptions};
use image;
use crate::{
    world::World,
    pixel::{Pixel, Color},
    ray::Ray,
    vector::Vector,
    lambertian::Lambertian,
    raytree::*,
};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum UpdateStatus{
    NotFinished,
    Finished,
    AboutToExit
}

const FOV: f64 = 70.;
// const MAX_RAY_DEPTH: u32 = 0;
const MAX_RAY_DEPTH: u32 = 3;
const NUM_OF_REFLECTED_RAYS: usize = 100;
// How many rays should be send in each reflection
// i.e. 0.75 -> first reflection will have 0.75 * NUM_OF_REFLECTED_RAYS,
// second will have 0.75 of previous number and so on
const SCATTERED_RAYS_FALLOFF: f64 = 0.5;
const WIDTH: u32 = 400;
const HEIGHT: u32 = 300;
// const WIDTH: u32 = 200;
// const HEIGHT: u32 = 200;
const WIDTH_CHUNK: u32 = 20;
const HEIGHT_CHUNK: u32 = 20;

pub struct Camera{
    buffer: Vec<Pixel>,
    pub starting_point: Vector,
    pub direction: Vector,
    lambertian: Lambertian,
    arena: RayArena,
    window: Window,
    pub chunk_num: u32,
    pub total_num_of_rays: u64
}

impl Camera{
    pub fn new() -> Camera
    {
        Camera{
            buffer: vec![Pixel::new(); (WIDTH * HEIGHT) as usize],
            starting_point: Vector{x:0., y:0., z: 5.},
            direction: Vector{
                x: 0.,
                y: 0.,
                z: -1.
            },
            lambertian: Lambertian::new(NUM_OF_REFLECTED_RAYS),
            arena: RayArena::new(MAX_RAY_DEPTH),
            window: Window::new("nrtrt", WIDTH as usize, HEIGHT as usize, WindowOptions::default()).unwrap(),
            chunk_num: 0,
            total_num_of_rays: 0
        }
    }

    fn get_pixel(&mut self, x: u32, y: u32) -> Option<&mut Pixel>
    {
        self.buffer.get_mut((x + y * WIDTH) as usize)
    }

    pub fn update(&mut self) -> UpdateStatus
    {
        //If all chunks are rendered

        //TODO: change that!
        let mut temp_buffer: Vec<u32> = Vec::with_capacity((WIDTH * HEIGHT) as usize);
        for pixel in 0..(WIDTH * HEIGHT)
        {
            temp_buffer.push(self.buffer[pixel as usize].color.to_u32());
        }

        self.window.update_with_buffer(&temp_buffer).unwrap();
        match self.window.is_open() && !self.window.is_key_down(Key::Escape){
            false => UpdateStatus::AboutToExit,
            true => {
                if (WIDTH / WIDTH_CHUNK) * (HEIGHT / HEIGHT_CHUNK) <= self.chunk_num{
                    return UpdateStatus::Finished;
                }                
                return UpdateStatus::NotFinished;
                }
        }
    }

    pub fn save_image(&self){
        // Lame method but works
        let mut buffer = Vec::with_capacity((WIDTH * HEIGHT * 3) as usize);
        for pixel in self.buffer.iter(){
            let pixel_val = pixel.color.to_u32();
            buffer.push(((pixel_val >> 16) & 255) as u8);
            buffer.push(((pixel_val >> 8) & 255) as u8);
            buffer.push((pixel_val & 255) as u8);
        }
        image::save_buffer("image.png", &buffer, WIDTH, HEIGHT, image::RGB(8)).unwrap();
    }

    pub fn shoot_primary_rays(&mut self, world: &World)
    {
        let pixel_to_pixel_angle = FOV / WIDTH as f64;
        let first_pixel_angle_horizontal = (WIDTH as i32 / -2) as f64 * pixel_to_pixel_angle;
        let first_pixel_angle_vertical = (HEIGHT as i32 / -2) as f64 * pixel_to_pixel_angle;
        //Clear the arena
        self.arena.nodes.clear();

        let chunk_x = self.chunk_num % (WIDTH / WIDTH_CHUNK);
        let start_x = WIDTH_CHUNK * chunk_x;
        let end_x = WIDTH_CHUNK * (chunk_x + 1);
        let chunk_y = self.chunk_num / (WIDTH / WIDTH_CHUNK);
        let start_y = HEIGHT_CHUNK * chunk_y;
        let end_y = HEIGHT_CHUNK * (chunk_y + 1);

        for x in start_x..end_x
        {
            for y in start_y..end_y
            {
                let mut ray_direction = self.direction;
                ray_direction.rotate_y(first_pixel_angle_horizontal + pixel_to_pixel_angle * x as f64);   //Rotate ray horizontally
                ray_direction.rotate_x(first_pixel_angle_vertical + pixel_to_pixel_angle * y as f64);   //Rotate ray vertically


                let ray = Ray::new(&self.starting_point, &ray_direction);
                if let Some(_) = world.item_that_collide(&ray)
                {
                    // Create reflected rays and add them to the arena
                    let node_id = self.arena.add_node(NodeId::Root, &Ray::new(&self.starting_point, &ray_direction));
                    self.shoot_reflected_rays(world, &self.lambertian.get_offsets().clone(), node_id);
                    let color = self.calculate_node_color(world, node_id);
                    self.get_pixel(x, y).unwrap().color = color;
                    // Remove the rays to save space
                    self.total_num_of_rays += self.arena.nodes.len() as u64;
                    self.arena.remove_node_with_childs(node_id);
                }
                else
                {
                    self.get_pixel(x, y).unwrap().color = Color{r: 128, g: 218, b: 235};
                }
            }
        }
        self.chunk_num += 1;
    }

    fn shoot_reflected_rays(&mut self, world: &World, offsets: &Vec<Vector>, id: NodeId){
        let ray_node_opt = self.arena.get_node(id);
        //If parent exists
        if let Some(ray_node) = ray_node_opt{
            if ray_node.recursion_depth >= MAX_RAY_DEPTH{
                return;
            }
            //If the collision occurred
            if let Some((collision_shape, new_collision_point)) = world.item_that_collide(&ray_node.ray){
                // Calculate the number of required rays
                let mut num_of_rays = NUM_OF_REFLECTED_RAYS as f64;
                if ray_node.recursion_depth > 0{
                    let denominator = (ray_node.recursion_depth + 1) as f64 * SCATTERED_RAYS_FALLOFF;
                    num_of_rays /= denominator;
                }
//                println!("{} {}", num_of_rays, ray_node.recursion_depth);
                for (idx, offset) in offsets.iter().enumerate(){
                    if idx > num_of_rays as usize{
                        break;
                    }
                    let new_direction = collision_shape.normal_at_point(&new_collision_point).unwrap() + *offset;
                    let new_ray = Ray::new(&new_collision_point, &new_direction);
                    let new_node_id = self.arena.add_node(id, &new_ray);
                    self.shoot_reflected_rays(world, offsets, new_node_id);
                }
            }
        }
    }

    fn calculate_node_color(&self, world: &World, id: NodeId) -> Color{
        if let NodeId::Parent(_) = id{
            if let Some(node) = self.arena.get_node(id)
            {
                // If it is the last ray, calculate the light that is reaching this point
                if node.child.len() == 0{
                    let color = self.calculate_last_node_color(world, id);
                    return color;
                }
                else{
                    let mut result = self.calculate_last_node_color(world, id);
                    for child in node.child.iter(){
                        if let Some(child_node) = self.arena.get_node(NodeId::Parent(*child)){
                            result += self.calculate_node_color(world, NodeId::Parent(child_node.id)) * child_node.ray.direction.distance() * (1. / NUM_OF_REFLECTED_RAYS as f64);
                        }
                    }
                    return result;
                }
            }
        }
        Color::white()
    }

    fn calculate_last_node_color(&self, world: &World, id: NodeId) -> Color{

        if let Some(node) = self.arena.get_node(id){
            if let Some((item, collision_point)) = world.item_that_collide(&node.ray){
                let normal = item.normal_at_point(&collision_point).unwrap().normalized();

                let mut resulting_color = Color::new();
                for light in world.lights.iter(){
                    let ray = Ray::new(&collision_point, &(light.position - collision_point));
                    if let None = world.item_that_collide(&ray){
                        let angle = ray.direction.normalized().dot(normal);
                        if angle > 0.{
                            resulting_color += (light.color * angle) * item.color() * item.reflectivity();
                        }
                    }
                }
                return resulting_color;
            }
        }
        Color::new()
    }
}