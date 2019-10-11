use crate::ray::Ray;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum NodeId {
    Root,
    Invalid,
    Parent(u32)
}

#[derive(Debug)]
pub struct RayNode{
    pub id: u32,
    pub parent: NodeId,
    pub child: Vec<u32>,
    pub ray: Ray,
    pub recursion_depth: u32
}

impl RayNode{
    pub fn new(id: u32, parent: NodeId, ray: &Ray, recursion_depth: u32) -> RayNode{
        RayNode{
            id,
            parent,
            child: Vec::new(),
            ray: *ray,
            recursion_depth
        }
    }

    pub fn add_child(&mut self, child: u32){
        self.child.push(child);
    }
}

pub struct RayArena{
    pub nodes: HashMap<u32, RayNode>,
    pub max_recursion_depth: u32,
}

impl RayArena{
    pub fn new(max_recursion_depth: u32) -> RayArena{
        RayArena{
            nodes: HashMap::new(),
            max_recursion_depth,
        }
    }

    pub fn get_node(&self, node_id: NodeId) -> Option<&RayNode>{
        if let NodeId::Parent(id) = node_id{
            return self.nodes.get(&id);
        }
        None
    }

    pub fn get_mut_node(&mut self, node_id: NodeId) -> Option<&mut RayNode>{
        if let NodeId::Parent(id) = node_id{
            return self.nodes.get_mut(&id);
        }
        None
    }

    pub fn add_node(&mut self, parent: NodeId, ray: &Ray) -> NodeId{
        if let NodeId::Parent(_) = parent {
            if let Some(node) = self.get_node(parent) {
                let recursion_depth = node.recursion_depth + 1;
                if recursion_depth > self.max_recursion_depth{
                    return NodeId::Invalid;
                }
                let new_id = self.nodes.len() as u32;
                self.nodes.insert(new_id, RayNode::new(new_id, parent, ray, recursion_depth));
                self.get_mut_node(parent).unwrap().add_child(new_id);
                return NodeId::Parent(new_id);
            }
        }
        else if let NodeId::Root = parent{
            let new_id = self.nodes.len() as u32;
            self.nodes.insert(new_id,RayNode::new(new_id, NodeId::Root, ray, 0));
            return NodeId::Parent(new_id);
        }
        return NodeId::Invalid;
    }

    //Returns the ends (the nodes that have no childrens itself) of a given node
    #[allow(dead_code)]
    pub fn get_last_nodes(&self, id: NodeId) -> Vec<u32>{
        if let NodeId::Parent(_) = id{
            if let Some(node) = self.get_node(id){
                let mut result = Vec::<u32>::new();
                //If it has no childrens
                if node.child.len() == 0{
                    result.push(node.id);
                }
                else{
                    //Iterate over all childs and find its last nodes
                    for child_id in node.child.iter(){
                        result.extend(self.get_last_nodes(NodeId::Parent(*child_id)).iter());
                    }
                }
                return result;
            }
        }
        Vec::new()
    }

    pub fn remove_node_with_childs(&mut self, id: NodeId){
        if let NodeId::Parent(node_id) = id{
            let mut parent_id = None;
            if let Some(node) = self.get_node(id){
                parent_id = Some(node.parent);
                // Recursively remove all childs
                for child_id in node.child.clone().iter(){
                    self.remove_node_with_childs(NodeId::Parent(*child_id));
                }
            }
            // Remove the parent
            if let Some(parent_id) = parent_id{
                if let Some(parent_node) = self.get_mut_node(parent_id){
                    // Because remove_item is unstable
                    let index = parent_node.child.iter().position(|x| *x == node_id).unwrap();
                    parent_node.child.remove(index);
                }
            }
            // Remove item itself
            self.nodes.remove(&node_id);
        }
    }
}


#[cfg(test)]
mod test
{
    use crate::ray::Ray;
    use crate::raytree::{RayArena, NodeId};

    #[test]
    fn add_nodes(){
        let mut ray_arena = RayArena::new(10);
        let root_node = ray_arena.add_node(NodeId::Root, &Ray::new_empty());
        let new_node_id_1 = ray_arena.add_node(root_node, &Ray::new_empty());
        let new_node_id_2 = ray_arena.add_node(root_node, &Ray::new_empty());
        let new_node_id_3 = ray_arena.add_node(root_node, &Ray::new_empty());

        //Check if correct amount of nodes were added
        assert_eq!(ray_arena.nodes.len(), 4);

        //Check if all of three nodes were added to root node and the parent of root node is "Root"
        assert_eq!(ray_arena.get_node(root_node).unwrap().child.len(), 3);
        assert_eq!(ray_arena.get_node(root_node).unwrap().parent, NodeId::Root);

        //Check if new nodes have no childs
        assert_eq!(ray_arena.get_node(new_node_id_1).unwrap().child.len(), 0);
        assert_eq!(ray_arena.get_node(new_node_id_2).unwrap().child.len(), 0);
        assert_eq!(ray_arena.get_node(new_node_id_3).unwrap().child.len(), 0);

        //Check if parent of the new nodes is set to root_node
        assert_eq!(ray_arena.get_node(new_node_id_1).unwrap().parent, root_node);
    }

    #[test]
    fn add_nested_nodes(){
        let mut ray_arena = RayArena::new(10);

        let root_node = ray_arena.add_node(NodeId::Root, &Ray::new_empty());
        let child_1 = ray_arena.add_node(root_node, &Ray::new_empty());
        let child_2 = ray_arena.add_node(child_1, &Ray::new_empty());
        let child_3 = ray_arena.add_node(child_2, &Ray::new_empty());

        assert_eq!(ray_arena.get_node(root_node).unwrap().child.len(), 1);
        assert_eq!(ray_arena.get_node(child_1).unwrap().child.len(), 1);
        assert_eq!(ray_arena.get_node(child_2).unwrap().child.len(), 1);
        assert_eq!(ray_arena.get_node(child_3).unwrap().child.len(), 0);
    }

    #[test]
    fn check_max_recursion_depth(){
        let mut ray_arena = RayArena::new(2);

        let root_node = ray_arena.add_node(NodeId::Root, &Ray::new_empty());
        let child_1 = ray_arena.add_node(root_node, &Ray::new_empty());
        let child_2 = ray_arena.add_node(child_1, &Ray::new_empty());
        let child_3 = ray_arena.add_node(child_2, &Ray::new_empty());
        let child_4 = ray_arena.add_node(child_3, &Ray::new_empty());

        //Check if only three items were added
        assert_eq!(ray_arena.nodes.len(), 3);

        assert_eq!(root_node, NodeId::Parent(0));
        assert_eq!(child_1, NodeId::Parent(1));
        assert_eq!(child_2, NodeId::Parent(2));
        //When recursion depth will get above max_recursion_depth, add_node should return Invalid
        assert_eq!(child_3, NodeId::Invalid);
        assert_eq!(child_4, NodeId::Invalid);
    }

    #[test]
    fn get_last_nodes(){
        let mut ray_arena = RayArena::new(5);

        let root_node = ray_arena.add_node(NodeId::Root, &Ray::new_empty());
        let child_1 = ray_arena.add_node(root_node, &Ray::new_empty());
        let child_2 = ray_arena.add_node(root_node, &Ray::new_empty());
        let child_3 = ray_arena.add_node(child_1, &Ray::new_empty());
        let child_4 = ray_arena.add_node(child_3, &Ray::new_empty());
        let child_5 = ray_arena.add_node(child_1, &Ray::new_empty());
        let child_6 = ray_arena.add_node(child_2, &Ray::new_empty());

        /*
        Tree looks like this:

                    root_node
                    /       \
            child_1         child_2
            /      \            |
        child_3     child_5     child_6
            |
        child_4
        */
        assert_eq!(ray_arena.get_node(child_2).unwrap().child.len(), 1);
        println!("{:?}", ray_arena.get_node(child_2).unwrap());
        println!("{:?}", ray_arena.get_node(child_6).unwrap());
        assert_eq!(ray_arena.get_last_nodes(root_node), vec!(4, 5, 6));
        assert_eq!(ray_arena.get_last_nodes(child_1), vec!(4, 5));
        assert_eq!(ray_arena.get_last_nodes(child_2), vec!(6));
        assert_eq!(ray_arena.get_last_nodes(child_6), vec!(6));
    }
    
    #[test]
    fn remove_node_with_childs(){
        let mut ray_arena = RayArena::new(5);

        let root_node = ray_arena.add_node(NodeId::Root, &Ray::new_empty());
        let child_1 = ray_arena.add_node(root_node, &Ray::new_empty());
        let child_2 = ray_arena.add_node(root_node, &Ray::new_empty());
        let child_3 = ray_arena.add_node(child_1, &Ray::new_empty());
        let child_4 = ray_arena.add_node(child_3, &Ray::new_empty());
        let child_5 = ray_arena.add_node(child_1, &Ray::new_empty());
        let child_6 = ray_arena.add_node(child_2, &Ray::new_empty());

        /*
        Tree looks like this:

                    root_node
                    /       \
            child_1         child_2
            /      \            |
        child_3     child_5     child_6
            |
        child_4
        */

        ray_arena.remove_node_with_childs(child_1);
        assert!(ray_arena.get_node(child_1).is_none());
        assert!(ray_arena.get_node(child_3).is_none());
        assert!(ray_arena.get_node(child_4).is_none());
        assert!(ray_arena.get_node(child_5).is_none());
    
        assert!(ray_arena.get_node(root_node).is_some());
        assert!(ray_arena.get_node(child_2).is_some());
        assert!(ray_arena.get_node(child_6).is_some());

        assert_eq!(ray_arena.get_node(root_node).unwrap().child, vec!(2));
    }
}