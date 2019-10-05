use crate::vector::Vector;
use crate::ray::Ray;
use std::collections::HashMap;

pub struct RayArena{
    rays: HashMap<u32, Ray>,
    current_id: u32
}

impl RayArena{
    pub fn new() -> RayArena{
        RayArena{
            rays: HashMap::new(),
            current_id: 0
        }
    }

    pub fn add_ray(&mut self, ray: &Ray){
        self.current_id += 1;
        self.rays.insert(self.current_id, ray.clone());
    }

    pub fn get_ray(&self, id: u32) -> Option<&Ray>{
        self.rays.get(&id)
    }

    pub fn get_current_id(&self) -> u32{
        self.current_id
    }
}

#[cfg(test)]
mod test
{
    use crate::ray::Ray;
    use crate::rayarena::RayArena;
    use crate::vector::Vector;

    # [test]
    fn get_ray(){
        let ray = Ray::new(&Vector::new(), &Vector::new());
        let mut ray_arena = RayArena::new();
        ray_arena.add_ray(&ray);
        let current_id = ray_arena.get_current_id();

        if let Some(_) = ray_arena.get_ray(current_id) {
            //Ok
        }
        else {
            assert!(false);
        }
        if let Some(_) = ray_arena.get_ray(current_id + 1) {
            assert!(false);
        }
        else {
            //Ok
        }
    }
}