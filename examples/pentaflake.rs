//! Pentaflake — port of the Haskell diagrams gallery example.
//!
//! Haskell:
//! ```
//! grad = defaultRG & _RG . rGradStops .~ mkStops [(blue,0,1), (crimson,1,1)]
//!                  & _RG . rGradRadius1 .~ 50
//! pentaflake' 0 = regPoly 5 1 # lw none
//! pentaflake' n = appends pCenter (zip vs (repeat (rotateBy (1/2) pOutside)))
//!   where vs = iterateN 5 (rotateBy (1/5)) . (if odd n then negated else id) $ unitY
//!         pCenter  = pentaflake' (n-1)
//!         pOutside = pCenter # opacity (1.7 / fromIntegral n)
//! pentaflake n = pentaflake' n # fillTexture grad # bgFrame 4 silver
//! ```

use hagoromo::*;
use std::f64::consts::TAU;
use std::fs;

/// Rotate a 2D vector by `angle` radians.
fn rotate_v2(v: Vec2, angle: f64) -> Vec2 {
    let (s, c) = angle.sin_cos();
    Vec2::new(v.x * c - v.y * s, v.x * s + v.y * c)
}

/// Build the n-th level pentaflake, without gradient/background.
fn pentaflake_inner(n: u32) -> Diagram {
    if n == 0 {
        return reg_poly(5, 1.0).stroke_width(0.0);
    }

    let p_center = pentaflake_inner(n - 1);
    let p_outside = pentaflake_inner(n - 1);
    // Note: The Haskell image looks like it does not use opacity,
    // even though it was in the code.
    // .opacity(1.7 / n as f64);
    // rotateBy(1/2) = 180°
    let p_outside_rotated = p_outside.rotate_by(0.5);

    // vs = iterateN 5 (rotateBy 1/5) . (negated if odd) $ unitY
    // unitY in our y-down coords = (0, -1) (points visually up)
    let start = if n % 2 == 1 {
        Vec2::new(0.0, 1.0) // negated unitY → pointing visually down
    } else {
        Vec2::new(0.0, -1.0)
    };
    let vs: Vec<Vec2> = (0..5)
        .map(|i| rotate_v2(start, i as f64 * TAU / 5.0))
        .collect();

    appends(
        p_center,
        vs.into_iter().map(|v| (v, p_outside_rotated.clone())),
    )
}

fn main() {
    // Radial gradient: blue at center → crimson at r=50 (large radius → subtle shift)
    let crimson = Color::rgb(0.863, 0.078, 0.235);
    let grad = RadialGradient::new(
        50.0,
        vec![
            GradientStop {
                offset: 0.0,
                color: BLUE,
                opacity: 1.0,
            },
            GradientStop {
                offset: 1.0,
                color: crimson,
                opacity: 1.0,
            },
        ],
    );

    let diagram = pentaflake_inner(4).fill_gradient(grad);

    let opts = RenderOptions {
        padding: 4.0,
        background: Some(SILVER),
        default_stroke_width: THIN,
    };

    let svg = render_svg(&diagram, &opts);
    fs::write("pentaflake.svg", &svg).expect("failed to write pentaflake.svg");
    println!("Wrote pentaflake.svg ({} bytes)", svg.len());
}
