# ardent

**Ardent** is a GPU-accelerated, vector-first user interface toolkit written in Rust.

It is designed from the ground up to combine the power of real-time graphics with the elegance of declarative UI composition. Ardent brings together scalable vector rendering, a reactive event system, and precise layout tools to serve the needs of developers building modern, expressive interfaces — from diagram editors and creative tools to programmable environments and visual applications.

---

## ✨ What is Ardent?

Ardent is not a general-purpose GUI framework. It's not trying to clone the web, nor recreate the look and feel of traditional OS widgets. Instead, it's a focused set of tools for building **interactive, resolution-independent, and fully GPU-rendered interfaces** — the kind you'd expect in professional design software, custom dashboards, or interactive editors.

Inspired by the scalability of SVG, the composability of scene graphs, and the performance of `wgpu`, Ardent gives you full control over how things are drawn, laid out, and interacted with — without the overhead of a browser engine or heavyweight UI toolkit.

---

## 🧱 Project Structure

Ardent is organized as a modular Rust workspace. Each crate plays a clearly defined role in the system:


```
ardent/
├── crates/
│ ├── ardent_core # Scene graph, nodes, transforms, styles, events
│ ├── ardent_render # GPU tessellation + rendering via lyon + wgpu
│ ├── ardent_layout # Layout abstraction (flexbox/grid via Taffy)
│ ├── ardent_text # Text as vector paths using ttf-parser + rustybuzz
│ ├── ardent_input # Input system and hit testing
│ ├── ardent_svg # SVG file loader and translation to scene graph
│ ├── ardent_macros # Procedural macros (e.g., svg_bind!)
│ └── ardent_demo # Interactive demo app (dev-only)
```

---


Each of these components is built to be cleanly separable, testable, and reusable. You can embed Ardent into your own engine, extend it with custom shapes or input models, or strip it down to the minimal core needed for your application.

---

## 🔍 What Makes Ardent Different?

- **GPU-native from the start**: Ardent doesn't wrap a canvas — it _is_ the renderer. All drawing happens via `wgpu`, with vector geometry handled through `lyon` tessellation.

- **Scene graph-based**: Ardent models UI as a hierarchy of nodes, each with a shape, transform, style, and optional event logic. This makes it ideal for building editors, visual tools, or anything else that benefits from spatial nesting and precise control.

- **SVG-inspired layout and interaction**: Shapes are declarative, scalable, and expressive. You can load an SVG, bind Rust handlers to specific elements, and interact with it natively.

- **Text as vector paths**: Rather than rasterizing fonts, Ardent extracts and tessellates glyph outlines, giving you precise control over typography and allowing text to scale, transform, or animate like any other shape.

- **Macro-powered event binding**: Use `svg_bind!` to map `onclick="my_handler()"` in an SVG file to an actual Rust function, safely and statically.

---

## 📦 Crate Breakdown

### `ardent_core`
Defines the node structure that makes up your UI. Nodes can have children, transforms, styles, shapes, and event handlers. This is the heart of the scene graph.

### `ardent_render`
Handles GPU rendering via `wgpu`. It tessellates shapes into vertex buffers using `lyon` and pushes them to the screen. Future versions will support batching, caching, and complex effects.

### `ardent_layout`
Provides a layout system powered by `taffy`, supporting flexbox-like constraints. Each node can participate in layout computations, and updates are propagated through the graph.

### `ardent_text`
Implements scalable, shaped text rendering. Fonts are parsed with `ttf-parser` and shaped using `rustybuzz`, then converted to paths for rendering alongside other vector shapes.

### `ardent_input`
Implements hit-testing, pointer tracking, and event bubbling. Nodes can respond to hover, click, and custom gestures. Input is spatial and node-aware, not global or flat.

### `ardent_svg`
Allows loading SVG files and converting them into scene graph nodes. Supports mapping element IDs and event attributes to Rust-side logic.

### `ardent_macros`
Provides procedural macros, including `svg_bind!`, which bridges SVG-declared events with statically bound Rust functions.

### `ardent_demo`
Contains an example application used during development. Useful for debugging, experimentation, and showcasing features.

---

## 🛠 Intended Use Cases

Ardent is ideal for:
- Creative tools (vector editors, animation tools)
- Diagramming systems (mind maps, flow charts, UML)
- Custom dashboards with animated and interactive graphics
- In-app design environments (WYSIWYG editors, low-code tools)
- Educational and simulation UIs

It's **not** intended to replace GTK, Qt, or Tauri. If you're building a desktop app with menus, buttons, and tabs — you're probably better served elsewhere.

---

## 📘 License

Dual-licensed under MIT or Apache-2.0, at your option.
