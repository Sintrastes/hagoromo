//! Hagoromo logo: chalk drawing a naturality square on a chalkboard.

use hagoromo::*;
use std::f64::consts::TAU;

// ── Palette ───────────────────────────────────────────────────────────────────

const WOOD: Color = Color::rgb(0.50, 0.33, 0.15);
const WOOD_DARK: Color = Color::rgb(0.36, 0.22, 0.08);
const WOOD_LIGHT: Color = Color::rgb(0.63, 0.44, 0.22);
const BOARD: Color = Color::rgb(0.17, 0.19, 0.21);
const BOARD_EDGE: Color = Color::rgb(0.11, 0.13, 0.14);
const MARK: Color = Color::rgb(0.22, 0.25, 0.28); // old chalk marks
const CHALK_DRAW: Color = Color::rgb(0.86, 0.84, 0.80); // diagram lines/dots
const CHALK_TEXT: Color = Color::rgb(0.83, 0.81, 0.77); // diagram labels
const CHALK_MID: Color = Color::rgb(0.93, 0.91, 0.87); // chalk body
const CHALK_HI: Color = Color::rgb(0.99, 0.98, 0.96);
const CHALK_LO: Color = Color::rgb(0.72, 0.70, 0.66);
const CHALK_FRONT: Color = Color::rgb(0.89, 0.87, 0.83);
const CHALK_BACK: Color = Color::rgb(0.64, 0.62, 0.58);

// ── Diagram drawing helpers ───────────────────────────────────────────────────

/// Arrow from `from` to `to` in chalk-on-board style.
///
/// Compensates for `polygon()` centroid centering so the arrowhead tip
/// lands exactly on the target node perimeter.
fn draw_arrow(from: (f64, f64), to: (f64, f64)) -> Diagram {
    let (x0, y0) = from;
    let (x1, y1) = to;
    let dx = x1 - x0;
    let dy = y1 - y0;
    let len = (dx * dx + dy * dy).sqrt();
    let angle = dy.atan2(dx);
    let turns = angle / TAU;

    let node_r: f64 = 2.2;
    let ah: f64 = 3.5;
    let centroid_adj = ah * 2.0 / 3.0;

    let shaft = stroke_trail(hrule(len - 2.0 * node_r - ah).rotate_by(turns))
        .lc(CHALK_DRAW)
        .stroke_width(1.05)
        .translate(x0 + node_r * angle.cos(), y0 + node_r * angle.sin());

    let tx = x1 - node_r * angle.cos();
    let ty = y1 - node_r * angle.sin();
    let head = polygon(&[
        Point::new(0.0, 0.0),
        Point::new(-ah, ah * 0.40),
        Point::new(-ah, -ah * 0.40),
    ])
    .fc(CHALK_DRAW)
    .stroke_width(0.0)
    .rotate(angle)
    .translate(
        tx - centroid_adj * angle.cos(),
        ty - centroid_adj * angle.sin(),
    );

    shaft + head
}

fn node_dot(x: f64, y: f64) -> Diagram {
    circle(1.8).fc(CHALK_DRAW).stroke_width(0.0).translate(x, y)
}

fn clabel(x: f64, y: f64, s: &str) -> Diagram {
    text(s, 5.5).fc(CHALK_TEXT).translate(x, y)
}

/// Naturality square (commutative square).
///
/// Objects:  A (top-left), B (top-right), C (bottom-left), D (bottom-right)
/// Morphisms: f: A→B, g: C→D, α: A→C, β: B→D   (g∘α = β∘f)
///
/// D is placed at (-8, 22) — bottom-left area of the board — because the
/// chalk tip will rest there.
fn naturality_square() -> Diagram {
    let (ax, ay) = (-36.0, -2.0); // A top-left
    let (bx, by) = (-8.0, -2.0); // B top-right
    let (cx, cy) = (-36.0, 22.0); // C bottom-left
    let (dx, dy) = (-8.0, 22.0); // D bottom-right  ← chalk tip

    Diagram::empty()
        // Objects
        + node_dot(ax, ay)
        + node_dot(bx, by)
        + node_dot(cx, cy)
        + node_dot(dx, dy)
        // Morphisms
        + draw_arrow((ax, ay), (bx, by))  // f:  A→B  (top)
        + draw_arrow((cx, cy), (dx, dy))  // g:  C→D  (bottom, last drawn)
        + draw_arrow((ax, ay), (cx, cy))  // α:  A→C  (left)
        + draw_arrow((bx, by), (dx, dy))  // β:  B→D  (right)
        // Object labels (small, chalk-style)
        + clabel(ax - 7.0, ay - 1.5, "A")
        + clabel(bx + 4.0, by - 1.5, "B")
        + clabel(cx - 7.0, cy + 1.5, "C")
        + clabel(dx + 4.0, dy + 1.5, "D")
        // Morphism labels
        + clabel((ax + bx) / 2.0, ay - 7.5, "f")
        + clabel((cx + dx) / 2.0, cy + 8.0, "g")
        + clabel(ax - 9.5, (ay + cy) / 2.0, "\u{03B1}") // α
        + clabel(bx + 7.5, (by + dy) / 2.0, "\u{03B2}") // β
}

// ── Board ─────────────────────────────────────────────────────────────────────

fn ellipse_poly(rx: f64, ry: f64) -> Diagram {
    let pts: Vec<Point> = (0..28)
        .map(|i| {
            let t = i as f64 / 28.0 * TAU;
            Point::new(rx * t.cos(), ry * t.sin())
        })
        .collect();
    polygon(&pts)
}

fn chalkboard() -> Diagram {
    let frame = rect(120.0, 80.0).fc(WOOD).stroke_width(0.0);

    let grain = {
        let w = 120.0;
        Diagram::empty()
            + rect(w, 1.1)
                .fc(WOOD_DARK)
                .stroke_width(0.0)
                .translate(-2.0, -37.5)
            + rect(w, 0.7)
                .fc(WOOD_LIGHT)
                .stroke_width(0.0)
                .translate(1.0, -35.0)
            + rect(w, 0.5)
                .fc(WOOD_DARK)
                .stroke_width(0.0)
                .translate(-1.0, -33.5)
            + rect(w, 0.6)
                .fc(WOOD_DARK)
                .stroke_width(0.0)
                .translate(2.0, 33.5)
            + rect(w, 0.8)
                .fc(WOOD_LIGHT)
                .stroke_width(0.0)
                .translate(-1.0, 35.5)
            + rect(w, 1.0)
                .fc(WOOD_DARK)
                .stroke_width(0.0)
                .translate(1.0, 37.5)
            + rect(w, 0.4)
                .fc(WOOD_DARK)
                .stroke_width(0.0)
                .translate(-3.0, -18.0)
            + rect(w, 0.5)
                .fc(WOOD_LIGHT)
                .stroke_width(0.0)
                .translate(2.0, 0.0)
            + rect(w, 0.4)
                .fc(WOOD_DARK)
                .stroke_width(0.0)
                .translate(-1.0, 18.0)
    };

    let bevel = rect(106.0, 66.0).fc(BOARD_EDGE).stroke_width(0.0);
    let surface = rect(104.0, 64.0).fc(BOARD).stroke_width(0.0);

    let marks = {
        let c = MARK;
        let sw = 0.30;
        Diagram::empty()
            + stroke_trail(hrule(44.0))
                .lc(c)
                .stroke_width(sw)
                .translate(-15.0, -20.0)
            + stroke_trail(hrule(32.0).rotate_by(0.008))
                .lc(c)
                .stroke_width(sw)
                .translate(8.0, -8.0)
            + stroke_trail(hrule(38.0).rotate_by(-0.006))
                .lc(c)
                .stroke_width(sw)
                .translate(-12.0, 5.0)
            + stroke_trail(hrule(28.0))
                .lc(c)
                .stroke_width(sw)
                .translate(5.0, 18.0)
            + stroke_trail(hrule(20.0).rotate_by(0.010))
                .lc(c)
                .stroke_width(sw)
                .translate(-20.0, 26.0)
    };

    frame + grain + bevel + surface + marks
}

// ── Chalk ─────────────────────────────────────────────────────────────────────

fn chalk_shape(length: f64, r: f64) -> Diagram {
    let hl = length / 2.0;
    let ex = r * 0.27;

    let back = ellipse_poly(ex, r)
        .fc(CHALK_BACK)
        .stroke_width(0.0)
        .translate(-hl, 0.0);
    let body = rect(length, r * 2.0).fc(CHALK_MID).stroke_width(0.0);
    let hi = rect(length, r * 0.42)
        .fc(CHALK_HI)
        .stroke_width(0.0)
        .translate(0.0, -(r * 0.65));
    let lo = rect(length, r * 0.50)
        .fc(CHALK_LO)
        .stroke_width(0.0)
        .translate(0.0, r * 0.67);
    let front = ellipse_poly(ex, r)
        .fc(CHALK_FRONT)
        .stroke_width(0.0)
        .translate(hl, 0.0);

    back + body + hi + lo + front
}

// ── Main ──────────────────────────────────────────────────────────────────────

fn main() {
    let angle = (-38.0_f64).to_radians();

    let chalk = chalk_shape(92.0, 5.5).rotate(angle).translate(-30.0, 26.0);

    let shadow = rect(122.0, 82.0)
        .fc(Color::rgb(0.10, 0.10, 0.12))
        .stroke_width(0.0)
        .translate(3.5, 3.5);

    let diagram = shadow + chalkboard() + chalk;

    let opts = RenderOptions {
        padding: 14.0,
        background: Some(Color::rgb(0.82, 0.79, 0.74)),
        default_stroke_width: Measure::Absolute(0.0),
    };

    let svg = render_svg(&diagram, &opts);
    std::fs::write("logo.svg", &svg).unwrap();
    eprintln!("Written to logo.svg  (bbox = {:?})", diagram.bbox().rect());
}
