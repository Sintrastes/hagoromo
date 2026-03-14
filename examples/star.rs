//! Star — string-art star pattern from the Haskell diagrams gallery.
//!
//! Each quarter is a fan of lines connecting evenly-spaced points on the
//! x-axis to evenly-spaced points on the y-axis, with alternating colors.
//! Four rotated copies fill all four screen quadrants.

use hagoromo::*;
use std::fs;

fn main() {
    let n = 20usize;
    let colors = [Color::from_hex("#5E0042"), Color::from_hex("#00856A")];

    // One quarter: lines from (k, 0) to (0, n-k) for k = 0..=n.
    // Lives in screen quadrant I (positive x, positive y = lower-right).
    let q: Diagram = (0..=n).fold(Diagram::empty(), |acc, k| {
        let from = Point::new(k as f64, 0.0);
        let to = Point::new(0.0, (n - k) as f64);
        acc + polyline(&[from, to]).lc(colors[k % 2])
    });

    // Four rotated copies fill all four quadrants around the origin.
    let star = q.clone()                   // lower-right  [0,n]×[0,n]
        + q.clone().rotate_by(-0.25)       // upper-right  [0,n]×[-n,0]
        + q.clone().rotate_by(0.5)         // upper-left  [-n,0]×[-n,0]
        + q.clone().rotate_by(0.25); // lower-left  [-n,0]×[0,n]

    // Whitesmoke background matching the Haskell gallery (CSS #F5F5F5).
    let bg = square(50.0)
        .fc(Color::from_hex("#F5F5F5"))
        .stroke_width(0.0);
    let diagram = bg + star;

    let opts = RenderOptions {
        padding: 2.5,
        background: Some(Color::rgb(1.0, 1.0, 1.0)),
        default_stroke_width: MEDIUM,
    };

    let svg = render_svg(&diagram, &opts);
    fs::write("star.svg", &svg).expect("failed to write star.svg");
    println!("Wrote star.svg ({} bytes)", svg.len());
}
