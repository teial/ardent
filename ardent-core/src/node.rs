use std::sync::atomic::{AtomicU64, Ordering};

use crate::event::EventHandler;
use crate::shape::Shape;
use crate::style::Style;
use crate::transform::Transform;

/// A unique identifier for a node within the scene graph.
///
/// Every node has a stable `NodeId` that distinguishes it from other nodes.
/// These are used for parenting, traversal, event routing, and lookup.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub u64);

/// Generates a new globally unique `NodeId`.
fn generate_id() -> NodeId {
    static COUNTER: AtomicU64 = AtomicU64::new(1);
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    NodeId(id)
}

/// A node in the scene graph representing a visual or interactive element.
///
/// Nodes are the primary building blocks of the user interface. Each one can
/// define a visual shape, styling information (e.g., fill color), spatial
/// transformation (position, scale, rotation), and optional event handlers
/// for interactivity. Nodes are arranged in a tree structure to reflect
/// hierarchical relationships such as nesting, grouping, and layout.
///
/// Even without prior knowledge of vector graphics or GPU rendering, you can
/// think of each node as a "box" in a tree, which may contain a shape, be
/// styled, respond to events, and contain other boxes.
pub struct Node {
    /// Unique identifier of the node.
    id: NodeId,

    /// Optional ID of the parent node.
    parent: Option<NodeId>,

    /// IDs of the child nodes.
    children: Vec<NodeId>,

    /// Position, scale, and rotation relative to the parent node.
    transform: Transform,

    /// Optional geometric shape (e.g., rectangle).
    shape: Option<Shape>,

    /// Visual styling (e.g., fill color).
    style: Style,

    /// Optional event handler function.
    on_event: Option<EventHandler>,

    /// Dirty flag for re-rendering.
    dirty: bool,
}

impl Node {
    /// Creates a new scene graph node with a unique ID and default properties.
    ///
    /// By default, the node has no shape or parent, no styling, and no
    /// event handler. It is initialized with an identity transform and an
    /// empty list of children. The node is marked as dirty initially.
    ///
    /// This method is useful when building a new scene from scratch.
    pub fn new() -> Self {
        Self {
            id: generate_id(),
            parent: None,
            children: Vec::new(),
            transform: Transform::default(),
            shape: None,
            style: Style::default(),
            on_event: None,
            dirty: true,
        }
    }

    /// Returns the globally unique identifier of this node.
    ///
    /// `NodeId` values are assigned automatically when a node is created,
    /// and they remain stable and unique for the lifetime of the node.
    pub fn id(&self) -> NodeId {
        self.id
    }

    /// Returns the ID of the parent node, if this node has a parent.
    ///
    /// Parent-child relationships define the scene graph structure and
    /// determine rendering and layout hierarchy.
    pub fn parent(&self) -> Option<NodeId> {
        self.parent
    }

    /// Sets the parent of this node.
    ///
    /// Normally called internally when inserting the node into the graph.
    pub fn set_parent(&mut self, parent: NodeId) {
        self.parent = Some(parent);
    }

    /// Returns a read-only view of this node’s child IDs.
    ///
    /// The children are rendered and processed in the order they appear here.
    pub fn children(&self) -> &[NodeId] {
        &self.children
    }

    /// Appends a child node ID to this node’s child list.
    ///
    /// You typically do this after creating the child node and inserting
    /// it into the scene graph.
    pub fn add_child(&mut self, child: NodeId) {
        self.children.push(child);
    }

    /// Removes a child node ID if present.
    ///
    /// This does not delete the actual node from the scene graph — it only
    /// detaches the reference from this parent node.
    pub fn remove_child(&mut self, child: NodeId) {
        self.children.retain(|&id| id != child);
    }

    /// Returns a reference to the node's 2D transform (position/scale/rotation).
    ///
    /// Transforms are applied relative to the parent node's coordinate system.
    /// Use this to read the current transform values.
    pub fn transform(&self) -> &Transform {
        &self.transform
    }

    /// Returns a mutable reference to the node's transform.
    ///
    /// Use this to modify the position, scale, or rotation of the node
    /// in its parent's coordinate space.
    pub fn transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }

    /// Returns the node’s geometric shape, if one is set.
    ///
    /// Shapes define what is visually rendered for this node.
    /// Examples include rectangles, circles, and paths.
    pub fn shape(&self) -> Option<&Shape> {
        self.shape.as_ref()
    }

    /// Sets the shape to be rendered for this node.
    ///
    /// The shape is drawn using the styling information provided by `style()`.
    pub fn set_shape(&mut self, shape: Shape) {
        self.shape = Some(shape);
    }

    /// Clears the shape from this node.
    ///
    /// After calling this, the node will no longer be visually rendered.
    pub fn clear_shape(&mut self) {
        self.shape = None;
    }

    /// Returns a reference to this node's style properties.
    ///
    /// Style affects how the shape is rendered — e.g., fill color or stroke.
    pub fn style(&self) -> &Style {
        &self.style
    }

    /// Returns a mutable reference to this node's style.
    ///
    /// Use this to update fill color, stroke color, or other styling parameters.
    pub fn style_mut(&mut self) -> &mut Style {
        &mut self.style
    }

    /// Assigns an event handler to this node.
    ///
    /// This allows the node to respond to user interaction like mouse clicks
    /// or hover events.
    pub fn set_event_handler(&mut self, handler: EventHandler) {
        self.on_event = Some(handler);
    }

    /// Removes the event handler from this node.
    pub fn clear_event_handler(&mut self) {
        self.on_event = None;
    }

    /// Returns `true` if the node is marked as dirty.
    ///
    /// Dirty nodes are those that have changed and need to be redrawn.
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Marks this node as dirty, indicating it needs to be redrawn.
    ///
    /// Typically triggered when a property like shape, style, or transform changes.
    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    /// Clears the dirty flag, indicating the node has been rendered.
    pub fn clear_dirty(&mut self) {
        self.dirty = false;
    }
}

impl Default for Node {
    fn default() -> Self {
        Self::new()
    }
}
