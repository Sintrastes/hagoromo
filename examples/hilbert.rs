//! Hilbert curve — a direct translation of the Haskell diagrams example:
//!
//! ```haskell
//! hilbert 0 = mempty
//! hilbert n = hilbert' (n-1) # reflectY <> vrule 1
//!          <> hilbert  (n-1) <> hrule 1
//!          <> hilbert  (n-1) <> vrule (-1)
//!          <> hilbert' (n-1) # reflectX
//!   where
//!     hilbert' m = hilbert m # rotateBy (1/4)
//!
//! diagram = strokeT (hilbert 6) # lc silver # opacity 0.3
//! ```

use hagoromo::{hrule, render_svg, stroke_trail, vrule, Measure, RenderOptions, Trail, SILVER};

fn hilbert(n: u32) -> Trail {
    if n == 0 {
        return Trail::default();
    }
    hilbert_prime(n - 1).reflect_y()
        + vrule(1.0)
        + hilbert(n - 1)
        + hrule(1.0)
        + hilbert(n - 1)
        + vrule(-1.0)
        + hilbert_prime(n - 1).reflect_x()
}

fn hilbert_prime(n: u32) -> Trail {
    hilbert(n).rotate_by(0.25)
}

fn main() {
    let trail = hilbert(6);
    eprintln!("Hilbert(6): {} segments", trail.len());

    let diagram = stroke_trail(trail).lc(SILVER).opacity(0.3);

    let opts = RenderOptions { default_stroke_width: Measure::Absolute(0.4), ..Default::default() };
    let svg = render_svg(&diagram, &opts);

    print!("{svg}");
}
