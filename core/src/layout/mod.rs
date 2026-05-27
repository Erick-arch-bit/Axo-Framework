use std::collections::HashMap;
use taffy::prelude::*;
use taffy::{TaffyTree, NodeId};

pub struct Engine {
    pub tree: TaffyTree,
    colors: HashMap<NodeId, [f32; 4]>,
    types: HashMap<NodeId, String>,
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

impl Engine {
    pub fn new() -> Self {
        Engine { tree: TaffyTree::new(), colors: HashMap::new(), types: HashMap::new() }
    }

    pub fn create_leaf(&mut self, style: Style, color: [f32; 4], node_type: &str) -> NodeId {
        let id = self.tree.new_leaf(style).unwrap();
        self.colors.insert(id, color);
        self.types.insert(id, node_type.to_string());
        id
    }

    pub fn create_container(&mut self, style: Style, children: &[NodeId], color: [f32; 4], node_type: &str) -> NodeId {
        let id = self.tree.new_with_children(style, children).unwrap();
        self.colors.insert(id, color);
        self.types.insert(id, node_type.to_string());
        id
    }

    pub fn compute(&mut self, root: NodeId, viewport_width: f32, viewport_height: f32) {
        self.tree
            .compute_layout(root, Size { width: AvailableSpace::Definite(viewport_width), height: AvailableSpace::Definite(viewport_height) })
            .unwrap();
    }

    pub fn get_layout(&self, node: NodeId) -> Layout {
        self.tree.layout(node).unwrap().to_owned()
    }

    pub fn get_color(&self, node: NodeId) -> [f32; 4] {
        self.colors.get(&node).copied().unwrap_or([0.5, 0.5, 0.5, 1.0])
    }

    pub fn set_color(&mut self, node: NodeId, color: [f32; 4]) {
        self.colors.insert(node, color);
    }

    pub fn get_type(&self, node: NodeId) -> String {
        self.types.get(&node).cloned().unwrap_or_default()
    }

    pub fn visit<F>(&self, node: NodeId, f: &mut F)
    where
        F: FnMut(NodeId, &Layout),
    {
        if let Ok(layout) = self.tree.layout(node) {
            f(node, layout);
        }
        if let Ok(children) = self.tree.children(node) {
            for child in children {
                self.visit(child, f);
            }
        }
    }
}
