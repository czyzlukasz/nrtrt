use crate::ray::Ray;
use std::cell::RefCell;
use std::rc::Rc;
use std::ops::DerefMut;

#[derive(Debug)]
pub struct RayTree{
    pub parent: Option<Rc<RayTree>>,
    pub child: Vec<Rc<RayTree>>,
    pub ray: Ray,
    pub recursion_depth: u32
}

impl RayTree{
    pub fn new(ray: Ray, recursion_depth: u32) -> RayTree{
        RayTree{
            parent: None,
            child: Vec::new(),
            ray,
            recursion_depth
        }
    }

    pub fn set_parent(&mut self, parent: &Rc<RayTree>){
        self.parent = Some(parent.clone());
    }

    pub fn add_child(&mut self, ray: &Ray) -> &mut RayTree{
        self.child.push(Rc::new(RayTree::new(*ray, self.recursion_depth + 1)));
        self.child.last_mut().unwrap()
    }
}

#[cfg(test)]
mod test{
    use crate::raytree::RayTree;
    use crate::ray::Ray;
    use crate::vector::Vector;
    use std::rc::Rc;

    #[test]
    fn set_parent(){
        let parent_item = RayTree::new(Ray::new(&Vector{x: 1., y: 2., z: 3.}, &Vector::new()), 0);
        let mut tree_item = RayTree::new(Ray::new(&Vector::new(), &Vector::new()), 0);

        tree_item.set_parent(&Rc::new(parent_item));
        let ray = tree_item.parent.unwrap().ray.start_position;
        assert_eq!(ray.x, 1.);
        assert_eq!(ray.y, 2.);
        assert_eq!(ray.z, 3.);
    }

    #[test]
    fn add_child(){
        let mut parent_item = RayTree::new(Ray::new(&Vector{x: 1., y: 2., z: 3.}, &Vector::new()), 0);
        let child_item1 = Ray::new(&Vector{x: 1., y: 2., z: 3.}, &Vector::new());
        let child_item2 = Ray::new(&Vector{x: 1., y: 2., z: 3.}, &Vector::new());
        let child_item3 = Ray::new(&Vector{x: 1., y: 2., z: 3.}, &Vector::new());

        let &mut child_ray_tree = parent_item.add_child(&child_item1);
        child_ray_tree.add_child(&child_item2);
        child_ray_tree.add_child(&child_item3);

        assert_eq!(parent_item.child.len(), 1);
        assert_eq!(child_ray_tree.child.len(), 2);
    }
}