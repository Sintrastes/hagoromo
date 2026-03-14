//! Tangent and Normal Vectors — port of the Haskell diagrams gallery example.
//!
//! Haskell: `cubicSpline False pts` with natural boundary conditions;
//! tangent/normal lines drawn as `symmetricLine v = fromOffsets [2 *^ v] # center`;
//! labels placed at `pt + tangentVector` / `pt + normalVector`.

use hagoromo::*;
use std::fs;

fn main() {
    // Same points as the Haskell example; negate y for screen coords (y-down).
    let pts = &[
        Point::new(0.0,  0.0),
        Point::new(1.0, -1.0),
        Point::new(2.0, -1.0),
        Point::new(3.0,  0.0),
        Point::new(3.5,  0.0),
    ];
    let spline = cubic_spline(pts);

    let param = 0.45_f64;
    let pt    = spline.at_param(param);
    let tv    = spline.tangent_at_param(param);  // raw (unnormalized) tangent
    let nv    = spline.normal_at_param(param);   // 90° CCW from tangent

    // symmetricLine v = fromOffsets [2 *^ v] # center  →  line from −v to +v
    let tangent_line = polyline(&[
        Point::new(-tv.x, -tv.y),
        Point::new( tv.x,  tv.y),
    ]);

    let normal_line = polyline(&[
        Point::new(-nv.x, -nv.y),
        Point::new( nv.x,  nv.y),
    ]);

    // rightAngleSquare = square 0.1 # alignBL # rotate (signedAngleBetween tv unitX)
    let s = 0.1;
    let angle = tv.y.atan2(tv.x);
    let right_angle_sq = square(s)
        .translate(s / 2.0, -s / 2.0)
        .rotate(angle);

    // Labels at the vector endpoint from the curve point:
    //   baselineText "tangent" # translate tangentVector # moveTo pt
    //   topLeftText  "normal"  # translate normalVector  # moveTo pt
    let font_size = 0.2;
    let tangent_label = text("tangent", font_size)
        .translate(pt.x + tv.x, pt.y + tv.y);
    let normal_label = text("normal", font_size)
        .translate(pt.x + nv.x, pt.y + nv.y);

    // Move the intersection group to the curve point.
    let indicators = (tangent_line + normal_line + right_angle_sq)
        .translate(pt.x, pt.y)
        + tangent_label
        + normal_label;

    let dot = circle(0.05)
        .fc(BLACK)
        .stroke_width(0.0)
        .translate(pt.x, pt.y);

    let curve = stroke_spline(&spline);

    let diagram = curve + indicators + dot;

    let opts = RenderOptions {
        padding: 0.5,
        background: Some(Color::rgb(1.0, 1.0, 1.0)),
        default_stroke_width: THIN,
    };

    let svg = render_svg(&diagram, &opts);
    fs::write("tangent.svg", &svg).expect("failed to write tangent.svg");
    println!("Wrote tangent.svg ({} bytes)", svg.len());
}
