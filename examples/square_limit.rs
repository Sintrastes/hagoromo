//! Square Limit — M.C. Escher's "Square Limit" woodcut via Henderson's functional geometry.
//!
//! Based on the Haskell diagrams gallery example by Jeremy Gibbons (Oxford University).
//! Reference: Peter Henderson, "Functional Geometry" (1982).
//!
//! The four fish tiles (P, Q, R, S) are defined as polylines in a 16×16 grid.
//! Tile coordinates follow Haskell's y-up convention; we flip y when constructing diagrams.

use hagoromo::*;
use std::fs;

// ---------------------------------------------------------------------------
// Tile markings in Haskell's (x, y) y-up 16×16 coordinate system.
// Each tile is a list of open polylines (list of [x, y] vertices).
// ---------------------------------------------------------------------------

const MARKINGS_P: &[&[[f64; 2]]] = &[
    &[[4.0, 4.0], [6.0, 0.0]],
    &[[0.0, 3.0], [3.0, 4.0], [0.0, 8.0], [0.0, 3.0]],
    &[[4.0, 5.0], [7.0, 6.0], [4.0, 10.0], [4.0, 5.0]],
    &[[11.0, 0.0], [10.0, 4.0], [8.0, 8.0], [4.0, 13.0], [0.0, 16.0]],
    &[[11.0, 0.0], [14.0, 2.0], [16.0, 2.0]],
    &[[10.0, 4.0], [13.0, 5.0], [16.0, 4.0]],
    &[[9.0, 6.0], [12.0, 7.0], [16.0, 6.0]],
    &[[8.0, 8.0], [12.0, 9.0], [16.0, 8.0]],
    &[[8.0, 12.0], [16.0, 10.0]],
    &[[0.0, 16.0], [6.0, 15.0], [8.0, 16.0], [12.0, 12.0], [16.0, 12.0]],
    &[[10.0, 16.0], [12.0, 14.0], [16.0, 13.0]],
    &[[12.0, 16.0], [13.0, 15.0], [16.0, 14.0]],
    &[[14.0, 16.0], [16.0, 15.0]],
];

const MARKINGS_Q: &[&[[f64; 2]]] = &[
    &[[2.0, 0.0], [4.0, 5.0], [4.0, 7.0]],
    &[[4.0, 0.0], [6.0, 5.0], [6.0, 7.0]],
    &[[6.0, 0.0], [8.0, 5.0], [8.0, 8.0]],
    &[[8.0, 0.0], [10.0, 6.0], [10.0, 9.0]],
    &[[10.0, 0.0], [14.0, 11.0]],
    &[
        [12.0, 0.0],
        [13.0, 4.0],
        [16.0, 8.0],
        [15.0, 10.0],
        [16.0, 16.0],
        [12.0, 10.0],
        [6.0, 7.0],
        [4.0, 7.0],
        [0.0, 8.0],
    ],
    &[[13.0, 0.0], [16.0, 6.0]],
    &[[14.0, 0.0], [16.0, 4.0]],
    &[[15.0, 0.0], [16.0, 2.0]],
    &[[0.0, 10.0], [7.0, 11.0]],
    &[[9.0, 12.0], [10.0, 10.0], [12.0, 12.0], [9.0, 12.0]],
    &[[8.0, 15.0], [9.0, 13.0], [11.0, 15.0], [8.0, 15.0]],
    &[[0.0, 12.0], [3.0, 13.0], [7.0, 15.0], [8.0, 16.0]],
    &[[2.0, 16.0], [3.0, 13.0]],
    &[[4.0, 16.0], [5.0, 14.0]],
    &[[6.0, 16.0], [7.0, 15.0]],
];

const MARKINGS_R: &[&[[f64; 2]]] = &[
    &[[0.0, 12.0], [1.0, 14.0]],
    &[[0.0, 8.0], [2.0, 12.0]],
    &[[0.0, 4.0], [5.0, 10.0]],
    &[[0.0, 0.0], [8.0, 8.0]],
    &[[1.0, 1.0], [4.0, 0.0]],
    &[[2.0, 2.0], [8.0, 0.0]],
    &[[3.0, 3.0], [8.0, 2.0], [12.0, 0.0]],
    &[[5.0, 5.0], [12.0, 3.0], [16.0, 0.0]],
    &[[0.0, 16.0], [2.0, 12.0], [8.0, 8.0], [14.0, 6.0], [16.0, 4.0]],
    &[[6.0, 16.0], [11.0, 10.0], [16.0, 6.0]],
    &[[11.0, 16.0], [12.0, 12.0], [16.0, 8.0]],
    &[[12.0, 12.0], [16.0, 16.0]],
    &[[13.0, 13.0], [16.0, 10.0]],
    &[[14.0, 14.0], [16.0, 12.0]],
    &[[15.0, 15.0], [16.0, 14.0]],
];

const MARKINGS_S: &[&[[f64; 2]]] = &[
    &[[0.0, 0.0], [4.0, 2.0], [8.0, 2.0], [16.0, 0.0]],
    &[[0.0, 4.0], [2.0, 1.0]],
    &[[0.0, 6.0], [7.0, 4.0]],
    &[[0.0, 8.0], [8.0, 6.0]],
    &[[0.0, 10.0], [7.0, 8.0]],
    &[[0.0, 12.0], [7.0, 10.0]],
    &[[0.0, 14.0], [7.0, 13.0]],
    &[[8.0, 16.0], [7.0, 13.0], [7.0, 8.0], [8.0, 6.0], [10.0, 4.0], [16.0, 0.0]],
    &[[10.0, 16.0], [11.0, 10.0]],
    &[[10.0, 6.0], [12.0, 4.0], [12.0, 7.0], [10.0, 6.0]],
    &[[13.0, 7.0], [15.0, 5.0], [15.0, 8.0], [13.0, 7.0]],
    &[[12.0, 16.0], [13.0, 13.0], [15.0, 9.0], [16.0, 8.0]],
    &[[13.0, 13.0], [16.0, 14.0]],
    &[[14.0, 11.0], [16.0, 12.0]],
    &[[15.0, 9.0], [16.0, 10.0]],
];

// ---------------------------------------------------------------------------
// Tile construction
// ---------------------------------------------------------------------------

/// Convert Haskell y-up (x,y) in [0,16]² to centered screen coords (x-8, 8-y).
fn h(xy: [f64; 2]) -> Point {
    Point::new(xy[0] - 8.0, 8.0 - xy[1])
}

fn make_tile(markings: &[&[[f64; 2]]]) -> Diagram {
    markings.iter().fold(Diagram::empty(), |acc, path| {
        let pts: Vec<Point> = path.iter().map(|&xy| h(xy)).collect();
        acc + polyline(&pts)
    })
}

fn blank_tile() -> Diagram {
    // Invisible 16×16 square — contributes bounding box but no visual output.
    rect(16.0, 16.0).stroke_width(0.0).opacity(0.0)
}

// ---------------------------------------------------------------------------
// Henderson combinators
// ---------------------------------------------------------------------------

fn rot(d: Diagram) -> Diagram {
    d.rotate_by(-0.25)
}

/// Arrange p and q side-by-side, squishing to maintain tile width.
fn hpair(p: Diagram, q: Diagram) -> Diagram {
    (p | q).center_x().center_y().scale_xy(0.5, 1.0)
}

/// Stack p above q, squishing to maintain tile height.
fn vpair(p: Diagram, q: Diagram) -> Diagram {
    (p / q).center_x().center_y().scale_xy(1.0, 0.5)
}

/// 2×2 grid of four tiles, scaled to single-tile size.
fn quartet(p: Diagram, q: Diagram, r: Diagram, s: Diagram) -> Diagram {
    ((p | q) / (r | s)).center_x().center_y().scale(0.5)
}

/// Asymmetric 2×2 arrangement used for the corner:
///   top-left:  p (double height), top-right: q (double size)
///   bot-left:  r (normal),        bot-right: s (double width)
/// Scaled to 1/3 to restore single-tile size.
fn skewquartet(p: Diagram, q: Diagram, r: Diagram, s: Diagram) -> Diagram {
    let top = p.scale_xy(1.0, 2.0) | q.scale(2.0);
    let bot = r | s.scale_xy(2.0, 1.0);
    (top / bot).center_x().center_y().scale(1.0 / 3.0)
}

/// Four rotations of `d` arranged in a 2×2 quartet.
fn cyc(d: Diagram) -> Diagram {
    let r1 = rot(d.clone());
    let r2 = rot(r1.clone());
    let r3 = rot(r2.clone());
    quartet(r1, d, r2, r3)
}

// ---------------------------------------------------------------------------
// Recursive corner construction (depth-limited unrolling of Haskell's
// corecursive `corner` definition)
// ---------------------------------------------------------------------------
//
//   corner = SkewQuartet p q r s  where
//     p  = VPair p' fishT
//     p' = HPair (Rot s) p          ← self-recursive via p
//     q  = Quartet p' q fishU s'    ← self-recursive via q
//     r  = fishQ
//     s  = HPair fishT s'
//     s' = VPair s (Rot³ p)         ← self-recursive via s, p
//
// We unfold to depth `d`; at depth 0 every recursive reference is blank.

struct Fish {
    t: Diagram, // quartet(fishP, fishQ, fishR, fishS)
    u: Diagram, // cyc(rot(fishQ))
    q: Diagram, // fishQ (used as `r` in corner)
}

/// p(d) = VPair (HPair (Rot s_{d-1}) p_{d-1}) fishT
fn cp(d: u32, f: &Fish) -> Diagram {
    if d == 0 {
        return blank_tile();
    }
    let s = cs(d - 1, f);
    let p = cp(d - 1, f);
    let p_prime = hpair(rot(s), p);
    vpair(p_prime, f.t.clone())
}

/// s(d) = HPair fishT (VPair s_{d-1} (Rot³ p_{d-1}))
fn cs(d: u32, f: &Fish) -> Diagram {
    if d == 0 {
        return blank_tile();
    }
    let s = cs(d - 1, f);
    let p = cp(d - 1, f);
    let sp = vpair(s, rot(rot(rot(p))));
    hpair(f.t.clone(), sp)
}

/// q(d) = Quartet (HPair (Rot s_{d-1}) p_{d-1}) q_{d-1} fishU (VPair s_{d-1} (Rot³ p_{d-1}))
fn cq(d: u32, f: &Fish) -> Diagram {
    if d == 0 {
        return blank_tile();
    }
    let s = cs(d - 1, f);
    let p = cp(d - 1, f);
    let p_prime = hpair(rot(s.clone()), p.clone());
    let q = cq(d - 1, f);
    let sp = vpair(s, rot(rot(rot(p))));
    quartet(p_prime, q, f.u.clone(), sp)
}

fn corner(d: u32, f: &Fish) -> Diagram {
    if d == 0 {
        return blank_tile();
    }
    let p = cp(d, f);
    let q = cq(d, f);
    let r = f.q.clone();
    let s = cs(d, f);
    skewquartet(p, q, r, s)
}

fn squarelimit(d: u32, f: &Fish) -> Diagram {
    cyc(corner(d, f))
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

fn main() {
    let fish_p = make_tile(MARKINGS_P);
    let fish_q = make_tile(MARKINGS_Q);
    let fish_r = make_tile(MARKINGS_R);
    let fish_s = make_tile(MARKINGS_S);

    let fish = Fish {
        t: quartet(fish_p.clone(), fish_q.clone(), fish_r.clone(), fish_s.clone()),
        u: cyc(rot(fish_q.clone())),
        q: fish_q,
    };

    let limit = squarelimit(2, &fish);

    // Sea-green semi-transparent overlay (matches the Haskell gallery example).
    // 14.5 < 15.33 (fish extent) so the fish bleed slightly over the green edges.
    let bg = square(14.5)
        .fc(Color::rgb(0.56, 0.74, 0.56))
        .stroke_width(0.0)
        .opacity(0.5);

    let diagram = bg + limit;

    // padding = -1/3 clips the viewport to ±(8 - 1/3) = ±7.667, matching the
    // fish extent exactly (blank layout tiles extend to ±8, but visible fish
    // only reach ±23/3; the SVG viewBox clips everything outside).
    let opts = RenderOptions {
        padding: -1.0 / 3.0,
        background: Some(Color::rgb(1.0, 1.0, 1.0)),
        default_stroke_width: Measure::Absolute(0.05),
    };

    let svg = render_svg(&diagram, &opts);
    fs::write("square_limit.svg", &svg).expect("failed to write square_limit.svg");
    println!("Wrote square_limit.svg ({} bytes)", svg.len());
}
