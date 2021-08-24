use indextree::{Arena, NodeId};
use nannou::wgpu::TextureView;
use nannou::window::Window;
use nannou::App;

///
/// Texture node that renders a texture
/// Can use multiple input textures
///
/// This can be part of [TextureTree]
///
pub trait TextureNode {
    fn update(&mut self, app: &App, dev: &Window, input: Vec<TextureView>);
    fn output(&self) -> TextureView;
}

///
/// TextureTree generates a Texture, based on a combination of [TextureNode]s.
///
/// Each node is part of the tree.
/// the children of a Node generate the input textures for its parent
pub struct TextureTree {
    arena: Arena<Box<dyn TextureNode>>,
    root: NodeId,
    node_stack: Vec<NodeId>,
}

impl TextureTree {
    /// makes a runnable Tree, this tree is not editable once created.
    ///
    /// **Parameter:**
    /// * arena [indextree::Arena] containing the tree structure
    /// * root: Root [NodeId] of the Tree
    pub fn new(arena: Arena<Box<dyn TextureNode>>, root: NodeId) -> Self {
        let node_stack = root.descendants(&arena).collect();
        Self {
            arena,
            root,
            node_stack,
        }
    }

    /// run the tree
    ///
    /// updates all [TextureNode]s inside the tree, beginning with the leaf, ending at the Root
    pub fn update(&mut self, app: &App, win: &Window) {
        //this order of update guaranies that all children are updated before its children
        for &n_id in self.node_stack.iter().rev() {
            // collect updated children when available
            let children_outputs: Vec<TextureView> = {
                n_id.children(&self.arena)
                    .map(|id| self.arena.get(id).unwrap().get().output())
                    .collect()
            };

            //get the node to update
            let node = { self.arena.get_mut(n_id).unwrap().get_mut() };

            // update the node
            node.update(app, win, children_outputs);
        }
    }

    /// get the texture output of the root node
    pub fn output(&self) -> TextureView {
        self.arena.get(self.root).unwrap().get().output()
    }
}
