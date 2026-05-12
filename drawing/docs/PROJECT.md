# Geometrical Shapes

Rust program that rasterizes lines, points, polygons, circles, and a wireframe cube onto a **1000×1000** PNG using the [raster](https://crates.io/crates/raster) crate. Each run uses random placement and colors (see the full guide for exact rules).

## What It Draws

Each run generates a fresh `image.png` (1000×1000 pixels, black background) containing:

| Shape | Count | Color | Construction |
|-------|-------|-------|--------------|
| Line | 1 | Random RGB per draw | Random endpoints on the canvas |
| Point | 1 | Random RGB per draw | Random position on the canvas |
| Rectangle | 1 | Random RGB per draw | Corners `(150, 300)` and `(50, 60)` |
| Triangle | 1 | Random RGB per draw | Vertices `(500, 500)`, `(250, 700)`, `(700, 800)` |
| Circles | 49 | Random RGB per circle | Random centers; radius uniform in **`[10, 400]`** pixels |
| Pentagon | 1 | Random RGB per draw | Vertices `(700, 100)`, `(800, 150)`, `(750, 250)`, `(650, 250)`, `(600, 150)` |
| Cube | 1 | Random RGB per draw | Front face `(100, 400)`–`(200, 500)`; back face `(150, 350)`–`(250, 450)` (see `main.rs` for all eight vertices) |

**Colors:** Each shape’s `Drawable::color()` uses `rand::thread_rng()` and samples each of R, G, B with `gen_range(30, 255)`, so channels are in **30..255** (30 through 254) and dark colors are avoided. Every call to `color()` can return a new random color; line-based shapes reuse one sampled color for all edges in a single `draw`.

## Functions & Methods

### `Point`

| Function | Signature | Description |
|----------|-----------|-------------|
| `new` | `(x: i32, y: i32) -> Point` | Creates a point at the given coordinates. |
| `random` | `(width: i32, height: i32) -> Point` | Creates a point at a random position within the canvas bounds. |

### `Line`

| Function | Signature | Description |
|----------|-----------|-------------|
| `new` | `(p1: &Point, p2: &Point) -> Line` | Creates a line between two points (coordinates copied). |
| `random` | `(width: i32, height: i32) -> Line` | Creates a line with two random endpoints within the canvas. |

### `Triangle`

| Function | Signature | Description |
|----------|-----------|-------------|
| `new` | `(p1: &Point, p2: &Point, p3: &Point) -> Triangle` | Triangle from three vertices; draws edges p1→p2→p3→p1. |

### `Rectangle`

| Function | Signature | Description |
|----------|-----------|-------------|
| `new` | `(p1: &Point, p2: &Point) -> Rectangle` | Rectangle from two corners; the other corners are derived internally. |

### `Circle`

| Function | Signature | Description |
|----------|-----------|-------------|
| `new` | `(center: &Point, radius: i32) -> Circle` | Circle at `center` with radius in pixels. |
| `random` | `(width: i32, height: i32) -> Circle` | Random center on the canvas; radius uniform in **[10, 400)**. |

### `Pentagon`

| Function | Signature | Description |
|----------|-----------|-------------|
| `new` | `(p1..p5: &Point) -> Pentagon` | Pentagon storing five vertices; edges are drawn in order and closed back to `p1`. |

### `Cube`

| Function | Signature | Description |
|----------|-----------|-------------|
| `new` | `(p1..p8: &Point) -> Cube` | Wireframe cube: eight vertices (front face, back face, and connecting edges). |

### Internal drawing helper

| Function | Signature | Description |
|----------|-----------|-------------|
| `draw_line` | `(image: &mut Image, p1: &Point, p2: &Point, color: Color)` | Private helper for line-based shapes. Uses [Bresenham's line algorithm](https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm). |

`Circle::draw` uses the [midpoint circle algorithm](https://en.wikipedia.org/wiki/Midpoint_circle_algorithm) (Bresenham-style, eight-way symmetry).

---

## Traits

### `Drawable`

Implemented by every shape:

```rust
fn draw(&self, image: &mut Image);
fn color(&self) -> Color;  // random RGB each call (see “Colors” above)
```

### `Displayable`

Implemented by `Image` in `main.rs`. Wraps `raster`’s `set_pixel` with a bounds check so drawing never panics on out-of-canvas coordinates.

```rust
fn display(&mut self, x: i32, y: i32, color: Color);
// skips pixels when x or y is outside [0, width) / [0, height)
```