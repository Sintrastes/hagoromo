//! Sierpinski triangle — a port of the Haskell diagrams gallery example.
//!
//! ```haskell
//! sierpinski n c = go n <> triangle (2^n) # fc (clrs !! 0) # lw none
//!   where
//!     go n
//!       | n == 1    = t1 # fc (clrs !! 1)
//!       | otherwise = appends tri (zip vecs (replicate 3 sierp))
//!       where
//!         tri   = scale (2^(n-1)) $ t1 # fc (clrs !! (n+1))
//!         sierp = go (n-1)
//!         t1    = triangle 1 # reflectY
//! ```
//!
//! Placement formula (screen coords, +y down, centroid at origin):
//!
//!   sub = side of sub-triangle = 2^(n−1)
//!   bottom-left  → translate(−sub/2,  sub·√3/6)
//!   bottom-right → translate(+sub/2,  sub·√3/6)
//!   top          → translate(0,       −sub·√3/3)

use hagoromo::*;

// ── Color palette (Purples, light → dark) ────────────────────────────────────

const PURPLES: [Color; 9] = [
    Color::rgb(0.988, 0.984, 0.992), // 0 near-white background
    Color::rgb(0.937, 0.929, 0.961), // 1
    Color::rgb(0.855, 0.812, 0.914), // 2
    Color::rgb(0.737, 0.675, 0.859), // 3
    Color::rgb(0.635, 0.494, 0.800), // 4
    Color::rgb(0.537, 0.318, 0.745), // 5
    Color::rgb(0.463, 0.165, 0.659), // 6
    Color::rgb(0.357, 0.067, 0.518), // 7
    Color::rgb(0.247, 0.000, 0.369), // 8 darkest
];

fn color(n: u32) -> Color {
    // Level n uses palette index clamped to valid range.
    // Outermost levels are darkest; smallest triangles are lightest.
    let idx = ((n + 1) as usize).min(PURPLES.len() - 1);
    PURPLES[idx]
}

// ── Sierpinski recursion ──────────────────────────────────────────────────────

/// Build a Sierpinski triangle of depth `n`.
///
/// The result is a filled equilateral triangle of side 2^n with the fractal
/// subdivisions, all centered at the origin. `n = 0` is a single triangle.
fn sierpinski(n: u32) -> Diagram {
    let side = (1u64 << n) as f64;

    // Background triangle at this level's color (fills the "hole" between subs).
    let bg = equilateral_triangle(side).fc(color(0)).stroke_width(0.0);

    let inner = go(n);

    bg + inner
}

fn go(n: u32) -> Diagram {
    let side = (1u64 << n) as f64;
    let tri = equilateral_triangle(side).fc(color(n)).stroke_width(0.0);

    if n == 0 {
        return tri;
    }

    // Sub-triangle side length.
    let sub = side / 2.0;

    // Offsets (see module-level doc comment for derivation).
    let dy_base = sub * 3_f64.sqrt() / 6.0; // sub · √3/6  (+y = down)
    let dy_top = sub * 3_f64.sqrt() / 3.0;  // sub · √3/3  (−y = up)

    let s = go(n - 1);

    tri                                            // fill this level's "hole"
        + s.clone().translate(-sub / 2.0, dy_base) // bottom-left
        + s.clone().translate(sub / 2.0, dy_base)  // bottom-right
        + s.translate(0.0, -dy_top)                // top
}

// ── Main ─────────────────────────────────────────────────────────────────────

fn main() {
    let depth = 7;
    let diagram = sierpinski(depth);

    let opts = RenderOptions { padding: 4.0, ..Default::default() };
    let svg = render_svg(&diagram, &opts);

    std::fs::write("sierpinski.svg", &svg).unwrap();
    eprintln!(
        "Sierpinski depth {depth}: bbox = {:?}",
        diagram.bbox().rect()
    );
    eprintln!("Written to sierpinski.svg");
}
