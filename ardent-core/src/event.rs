/// Represents a basic user interaction or input event.
///
/// Events describe interactions such as mouse clicks or hover changes.
/// These are dispatched to nodes that have registered event handlers.
///
/// Events are designed to be high-level and shape-aware — they are routed
/// to specific nodes based on hit-testing results, not as global signals.
#[derive(Debug, Clone, Copy)]
pub enum Event {
    /// The user clicked on the node (usually via left mouse button).
    Click,

    /// The pointer entered the node’s area (hover begin).
    PointerEnter,

    /// The pointer exited the node’s area (hover end).
    PointerLeave,
    // TODO:
    // DragStart, DragUpdate, DragEnd
    // KeyPress(char), Scroll(f32), etc.
}

/// A boxed callback that responds to an input `Event`.
///
/// Event handlers are stored in the scene graph per-node and invoked when
/// an event is dispatched to that node. They are required to be thread-safe
/// to allow parallel traversal and rendering.
///
/// The handler receives the event value and performs side effects
/// (e.g., state updates, signal writes).
pub type EventHandler = Box<dyn Fn(Event) + Send + Sync>;
