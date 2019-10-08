use crate::ray::Ray;

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
    pub nodes: Vec<RayNode>,
    pub max_recursion_depth: u32,
}

impl RayArena{
    pub fn new(max_recursion_depth: u32) -> RayArena{
        RayArena{
            nodes: Vec::new(),
            max_recursion_depth,
        }
    }

    pub fn get_node(&mut self, node_id: NodeId) -> Option<&RayNode>{
        if let NodeId::Parent(id) = node_id{
            return self.nodes.get(id as usize);
        }
        None
    }

    pub fn get_mut_node(&mut self, id: u32) -> Option<&mut RayNode>{
        self.nodes.get_mut(id as usize)
    }

    pub fn add_node(&mut self, parent: NodeId, ray: &Ray) -> NodeId{
        if let NodeId::Parent(parent_id) = parent {
            let parent_node = self.get_node(parent);
            if let Some(node) = parent_node {
                let recursion_depth = node.recursion_depth + 1;
                if recursion_depth > self.max_recursion_depth{
                    return NodeId::Invalid;
                }
                let new_id = self.nodes.len() as u32;
                self.nodes.push(RayNode::new(new_id, parent, ray, recursion_depth));
                self.get_mut_node(parent_id).unwrap().add_child(new_id);
                return NodeId::Parent(new_id);
            }
        }
        else if let NodeId::Root = parent{
            let new_id = self.nodes.len() as u32;
            self.nodes.push(RayNode::new(new_id, NodeId::Root, ray, 0));
            return NodeId::Parent(new_id);
        }
        return NodeId::Invalid;
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
}