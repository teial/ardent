use std::collections::HashMap;

use crate::node::{Node, NodeId};

/// A scene graph managing a tree of UI nodes.
///
/// The `Scene` struct owns and organizes all the nodes in a user interface.
/// It tracks relationships between nodes (parent/child) and provides methods
/// for traversal, mutation, and ID generation. Think of it as a lightweight
/// "DOM" or document model tailored for a GPU-accelerated vector UI system.
///
/// Unlike HTML or SVG, the `Scene` stores its nodes in a flat `HashMap` keyed
/// by `NodeId`, with explicit parent/child references to form a tree.
pub struct Scene {
    /// All nodes in the scene, indexed by their unique IDs.
    nodes: HashMap<NodeId, Node>,

    /// The root node of the scene.
    root: NodeId,
}

impl Scene {
    /// Creates a new scene graph with a single root node.
    ///
    /// The root node is always created automatically and serves as the top-most
    /// container in the hierarchy. You can attach other nodes to it as children
    /// using `add_node`.
    pub fn new() -> Self {
        let root = Node::new();
        let root_id = root.id();

        let mut nodes = HashMap::new();
        nodes.insert(root_id, root);

        Self {
            nodes,
            root: root_id,
        }
    }

    /// Returns the root node’s ID.
    ///
    /// This is useful if you need to attach a new node to the top level.
    pub fn root(&self) -> NodeId {
        self.root
    }

    /// Inserts a new node into the scene graph and attaches it to a parent.
    ///
    /// The child node must be constructed by the caller. This method sets the
    /// child's parent field, updates the parent’s children list, and stores the
    /// node in the internal registry.
    ///
    /// # Panics
    /// Panics if the `parent` node does not exist in the scene.
    pub fn add_node(&mut self, parent: NodeId, mut node: Node) {
        node.set_parent(parent);
        self.nodes
            .get_mut(&parent)
            .map(|parent_node| parent_node.add_child(node.id()))
            .unwrap_or_else(|| panic!("Parent node with ID {:?} not found", parent));
        self.nodes.insert(node.id(), node);
    }

    /// Removes a node and its entire subtree from the scene graph.
    ///
    /// This will recursively delete the node and all of its children,
    /// removing them from the internal registry and detaching them from
    /// their parent.
    pub fn remove_node(&mut self, node_id: NodeId) {
        if let Some(node) = self.nodes.remove(&node_id) {
            if let Some(parent_id) = node.parent() {
                if let Some(parent) = self.nodes.get_mut(&parent_id) {
                    parent.remove_child(node_id);
                }
            }
            for child_id in node.children() {
                self.remove_node(*child_id);
            }
        }
    }

    /// Returns a reference to the node with the given ID, if it exists.
    ///
    /// This is a read-only view and does not allow mutation.
    pub fn get_node(&self, node_id: NodeId) -> Option<&Node> {
        self.nodes.get(&node_id)
    }

    /// Returns a mutable reference to the node with the given ID, if it exists.
    ///
    /// Use this to update properties like transform, shape, or style.
    pub fn get_node_mut(&mut self, node_id: NodeId) -> Option<&mut Node> {
        self.nodes.get_mut(&node_id)
    }

    /// Traverses all nodes in the scene graph in depth-first order.
    ///
    /// This method is useful for operations like rendering, layout, or hit-testing.
    /// The traversal starts at the root node and visits children recursively.
    ///
    /// # Example
    /// ```rust
    /// scene.traverse(|node| {
    ///     println!("Node {:?}", node.id());
    /// });
    /// ```
    pub fn traverse<F: FnMut(&Node)>(&self, mut callback: F) {
        fn recurse<F: FnMut(&Node)>(scene: &Scene, node_id: NodeId, callback: &mut F) {
            if let Some(node) = scene.get_node(node_id) {
                callback(node);
                for &child_id in node.children() {
                    recurse(scene, child_id, callback);
                }
            }
        }
        recurse(self, self.root, &mut callback);
    }

    /// Traverses all nodes in the scene graph mutably in depth-first order.
    ///
    /// This is useful when modifying each node (e.g., during layout or style updates).
    pub fn traverse_mut<F: FnMut(&mut Node)>(&mut self, mut callback: F) {
        fn recurse<F: FnMut(&mut Node)>(scene: &mut Scene, node_id: NodeId, callback: &mut F) {
            if let Some(node) = scene.get_node_mut(node_id) {
                callback(node);
                let children = node.children().to_vec(); // clone to avoid borrow conflicts
                for child_id in children {
                    recurse(scene, child_id, callback);
                }
            }
        }
        let root = self.root;
        recurse(self, root, &mut callback);
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}
