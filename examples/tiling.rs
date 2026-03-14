//! Semiregular Plane Tilings — port of the Haskell diagrams gallery example.
//!
//! Haskell: `drawTiling t3464 10 10 # lc white # lw thick # centerXY # pad 1.1`
//!
//! Generates the 3-4-6-4 semiregular tiling (triangles, squares, and hexagons)
//! using the exact tiling zipper algorithm from Diagrams.TwoD.Tilings.

use hagoromo::*;
use std::collections::HashSet;
use std::fs;

// ── Tiling polygon types ──────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TilingPoly {
    Triangle,
    Square,
    Hexagon,
    Octagon,
    Dodecagon,
}

fn poly_sides(p: TilingPoly) -> usize {
    match p {
        TilingPoly::Triangle  => 3,
        TilingPoly::Square    => 4,
        TilingPoly::Hexagon   => 6,
        TilingPoly::Octagon   => 8,
        TilingPoly::Dodecagon => 12,
    }
}

fn poly_from_sides(n: usize) -> TilingPoly {
    match n {
        3  => TilingPoly::Triangle,
        4  => TilingPoly::Square,
        6  => TilingPoly::Hexagon,
        8  => TilingPoly::Octagon,
        12 => TilingPoly::Dodecagon,
        _  => panic!("bad polygon side count: {n}"),
    }
}

/// Cosine of the polygon's internal angle (exact, from Q[√2,√3]).
fn poly_cos(p: TilingPoly) -> f64 {
    match p {
        TilingPoly::Triangle  =>  0.5,
        TilingPoly::Square    =>  0.0,
        TilingPoly::Hexagon   => -0.5,
        TilingPoly::Octagon   => -2.0_f64.sqrt() / 2.0,  // -(1/2)√2
        TilingPoly::Dodecagon => -3.0_f64.sqrt() / 2.0,  // -(1/2)√3
    }
}

/// Sine of the polygon's internal angle.
fn poly_sin(p: TilingPoly) -> f64 {
    match p {
        TilingPoly::Triangle  =>  3.0_f64.sqrt() / 2.0,  // (1/2)√3
        TilingPoly::Square    =>  1.0,
        TilingPoly::Hexagon   =>  3.0_f64.sqrt() / 2.0,  // (1/2)√3
        TilingPoly::Octagon   =>  2.0_f64.sqrt() / 2.0,  // (1/2)√2
        TilingPoly::Dodecagon =>  0.5,
    }
}

/// Rotate v by the polygon's internal angle.
fn poly_rotation(p: TilingPoly, v: V2) -> V2 {
    let (x, y) = v;
    let c = poly_cos(p);
    let s = poly_sin(p);
    (x * c - y * s, x * s + y * c)
}

/// Rotate v by the polygon's exterior angle (= π − internal angle).
/// Matrix: [−c,−s; s,−c] = rotation by (π − θ).
fn poly_ext_rotation(p: TilingPoly, v: V2) -> V2 {
    let (x, y) = v;
    let c = poly_cos(p);
    let s = poly_sin(p);
    (-x * c - y * s, x * s - y * c)
}

/// Fill color for each polygon type.
fn poly_color(p: TilingPoly) -> Color {
    match p {
        TilingPoly::Triangle  => Color::rgb(1.000, 1.000, 0.000), // yellow
        TilingPoly::Square    => Color::rgb(0.235, 0.702, 0.443), // mediumseagreen
        TilingPoly::Hexagon   => Color::rgb(0.541, 0.169, 0.886), // blueviolet
        TilingPoly::Octagon   => Color::rgb(0.690, 0.769, 0.871), // lightsteelblue
        TilingPoly::Dodecagon => Color::rgb(0.392, 0.584, 0.929), // cornflowerblue
    }
}

// ── Tiling state machine ──────────────────────────────────────────────────────
//
// Mirrors the Haskell `Tiling { curConfig, follow }` corecursive structure as
// an explicit finite state machine.

#[derive(Clone, Debug)]
enum Tiling {
    /// t3 (6 triangles), t4 (4 squares), t6 (3 hexagons).
    Uniform(TilingPoly, usize),
    /// `semiregular ps trans`: state = current rotation into `ps`.
    Semiregular { ps: Vec<usize>, trans: Vec<usize>, state: usize },
    /// `mk3Tiling [a,b,c]`: state = current [a,b,c] permutation.
    Mk3 { abc: [usize; 3] },
    /// `t3636`: alternates between [3,6,3,6] and [6,3,6,3].
    T3636 { flipped: bool },
}

fn rot_vec(v: &[usize], by: usize) -> Vec<usize> {
    if v.is_empty() { return vec![]; }
    let by = by % v.len();
    [&v[by..], &v[..by]].concat()
}

impl Tiling {
    fn cur_config(&self) -> Vec<TilingPoly> {
        match self {
            Tiling::Uniform(p, n) => vec![*p; *n],
            Tiling::Semiregular { ps, state, .. } => {
                rot_vec(ps, *state).iter().map(|&n| poly_from_sides(n)).collect()
            }
            Tiling::Mk3 { abc } => {
                abc.iter().map(|&n| poly_from_sides(n)).collect()
            }
            Tiling::T3636 { flipped } => {
                if *flipped { [6usize, 3, 6, 3] } else { [3usize, 6, 3, 6] }
                    .iter().map(|&n| poly_from_sides(n)).collect()
            }
        }
    }

    fn follow(&self, i: usize) -> Tiling {
        match self {
            Tiling::Uniform(p, n) => Tiling::Uniform(*p, *n),
            Tiling::Semiregular { ps, trans, state } => {
                // follow(j) = mkT(rot(state, trans)[j])
                let rotated = rot_vec(trans, *state);
                let new_state = rotated[i % rotated.len()];
                Tiling::Semiregular { ps: ps.clone(), trans: trans.clone(), state: new_state }
            }
            Tiling::Mk3 { abc } => {
                let [a, b, c] = *abc;
                let new_abc = match i % 3 {
                    0 => [c, b, a],  // reverse
                    1 => [a, c, b],
                    _ => [b, a, c],
                };
                Tiling::Mk3 { abc: new_abc }
            }
            Tiling::T3636 { flipped } => {
                // even index → reverse (flip), odd index → same
                Tiling::T3636 { flipped: if i % 2 == 0 { !flipped } else { *flipped } }
            }
        }
    }
}

// ── Pre-defined tilings ───────────────────────────────────────────────────────

#[allow(dead_code)]
fn t3()     -> Tiling { Tiling::Uniform(TilingPoly::Triangle, 6) }
#[allow(dead_code)]
fn t4()     -> Tiling { Tiling::Uniform(TilingPoly::Square, 4) }
#[allow(dead_code)]
fn t6()     -> Tiling { Tiling::Uniform(TilingPoly::Hexagon, 3) }
#[allow(dead_code)]
fn t3636()  -> Tiling { Tiling::T3636 { flipped: false } }
#[allow(dead_code)]
fn t4612()  -> Tiling { Tiling::Mk3 { abc: [4, 6, 12] } }
#[allow(dead_code)]
fn t488()   -> Tiling { Tiling::Mk3 { abc: [4, 8, 8] } }
#[allow(dead_code)]
fn t31212() -> Tiling { Tiling::Mk3 { abc: [3, 12, 12] } }

fn semiregular(ps: &[usize], trans: &[usize]) -> Tiling {
    Tiling::Semiregular { ps: ps.to_vec(), trans: trans.to_vec(), state: 0 }
}

fn t3464()   -> Tiling { semiregular(&[4, 3, 4, 6],    &[3, 2, 1, 0]) }
#[allow(dead_code)]
fn t33434()  -> Tiling { semiregular(&[3, 4, 3, 4, 3], &[0, 2, 1, 4, 3]) }
#[allow(dead_code)]
fn t33344()  -> Tiling { semiregular(&[4, 3, 3, 3, 4], &[0, 4, 2, 3, 1]) }
#[allow(dead_code)]
fn t33336l() -> Tiling { semiregular(&[3, 3, 3, 3, 6], &[4, 1, 3, 2, 0]) }
#[allow(dead_code)]
fn t33336r() -> Tiling { semiregular(&[3, 3, 3, 3, 6], &[4, 2, 1, 3, 0]) }

// ── Tiling generation ─────────────────────────────────────────────────────────

type V2 = (f64, f64);

/// Integer key for vertex deduplication (1e7 factor → 7 decimal places).
fn v2_key(v: V2) -> (i64, i64) {
    ((v.0 * 1e7).round() as i64, (v.1 * 1e7).round() as i64)
}

fn edge_key(v1: V2, v2: V2) -> ((i64, i64), (i64, i64)) {
    let k1 = v2_key(v1);
    let k2 = v2_key(v2);
    if k1 <= k2 { (k1, k2) } else { (k2, k1) }
}

fn poly_key(vs: &[V2]) -> Vec<(i64, i64)> {
    let mut keys: Vec<_> = vs.iter().map(|&v| v2_key(v)).collect();
    keys.sort();
    keys
}

/// `genPolyVs`: generate polygon vertices given start vertex `v` and
/// first edge direction `d` (Haskell: `scanl (^+^) v . take (n-1) . iterate extRot $ d`).
fn gen_poly_vs(p: TilingPoly, v: V2, d: V2) -> Vec<V2> {
    let n = poly_sides(p);
    let mut vertices = vec![v];
    let mut dir = d;
    for _ in 0..n - 1 {
        let last = *vertices.last().unwrap();
        vertices.push((last.0 + dir.0, last.1 + dir.1));
        dir = poly_ext_rotation(p, dir);
    }
    vertices
}

/// `genNeighbors`: compute neighbor vertices and polygon vertex lists around
/// vertex `v` with incoming direction `d`.
///
/// Haskell: `mapAccumL (\d' poly -> (polyRotation poly d', (v+d', genPolyVs poly v d'))) (-d) curConfig`
fn gen_neighbors(t: &Tiling, v: V2, d: V2) -> (Vec<V2>, Vec<(Vec<V2>, TilingPoly)>) {
    let config = t.cur_config();
    let mut neighbors = Vec::with_capacity(config.len());
    let mut polys = Vec::with_capacity(config.len());
    let mut cur_d = (-d.0, -d.1); // start from negated d
    for poly in &config {
        let nb = (v.0 + cur_d.0, v.1 + cur_d.1);
        neighbors.push(nb);
        polys.push((gen_poly_vs(*poly, v, cur_d), *poly));
        cur_d = poly_rotation(*poly, cur_d);
    }
    (neighbors, polys)
}

struct Output {
    polygons: Vec<(Vec<V2>, TilingPoly)>,
    edges: Vec<(V2, V2)>,
}

fn generate_tiling(t: &Tiling, v: V2, d: V2, in_rect: &impl Fn(V2) -> bool) -> Output {
    let mut visited_verts: HashSet<(i64, i64)> = HashSet::new();
    let mut visited_edges: HashSet<((i64, i64), (i64, i64))> = HashSet::new();
    let mut visited_polys: HashSet<Vec<(i64, i64)>> = HashSet::new();
    let mut output = Output { polygons: vec![], edges: vec![] };
    generate_inner(
        t, v, d, in_rect,
        &mut visited_verts, &mut visited_edges, &mut visited_polys,
        &mut output,
    );
    output
}

fn generate_inner(
    t: &Tiling,
    v: V2,
    d: V2,
    in_rect: &impl Fn(V2) -> bool,
    visited_verts: &mut HashSet<(i64, i64)>,
    visited_edges: &mut HashSet<((i64, i64), (i64, i64))>,
    visited_polys: &mut HashSet<Vec<(i64, i64)>>,
    output: &mut Output,
) {
    if !in_rect(v) { return; }
    let vk = v2_key(v);
    if visited_verts.contains(&vk) { return; }
    visited_verts.insert(vk);

    let (neighbors, polys) = gen_neighbors(t, v, d);

    for &nb in &neighbors {
        let ek = edge_key(v, nb);
        if !visited_edges.contains(&ek) {
            visited_edges.insert(ek);
            output.edges.push((v, nb));
        }
    }

    for (vs, poly_type) in &polys {
        let pk = poly_key(vs);
        if !visited_polys.contains(&pk) {
            visited_polys.insert(pk);
            output.polygons.push((vs.clone(), *poly_type));
        }
    }

    for (i, nb) in neighbors.iter().enumerate() {
        let nb_d = (nb.0 - v.0, nb.1 - v.1);
        generate_inner(
            &t.follow(i), *nb, nb_d, in_rect,
            visited_verts, visited_edges, visited_polys, output,
        );
    }
}

// ── Drawing ───────────────────────────────────────────────────────────────────

/// Draw a filled polygon from absolute vertex positions.
///
/// `polygon()` centers at the centroid, so we center the vertices ourselves
/// then translate back.
fn draw_poly(vs: &[V2], color: Color) -> Diagram {
    let n = vs.len() as f64;
    let cx = vs.iter().map(|p| p.0).sum::<f64>() / n;
    let cy = vs.iter().map(|p| p.1).sum::<f64>() / n;
    let centered: Vec<Point> = vs.iter()
        .map(|p| Point::new(p.0 - cx, p.1 - cy))
        .collect();
    polygon(&centered).fc(color).stroke_width(0.0).translate(cx, cy)
}

fn draw_edge(v1: V2, v2: V2) -> Diagram {
    polyline(&[Point::new(v1.0, v1.1), Point::new(v2.0, v2.1)])
        .lc(WHITE)
        .stroke_width(0.05)
}

fn main() {
    // Corresponds to `drawTiling t3464 10 10`:
    // visit vertices within ±5 units of origin.
    let half = 5.0_f64;
    let in_rect = |v: V2| v.0 >= -half && v.0 <= half && v.1 >= -half && v.1 <= half;

    let out = generate_tiling(&t3464(), (0.0, 0.0), (1.0, 0.0), &in_rect);

    let mut diagram = Diagram::empty();

    // Polygons first (background layer).
    for (vs, poly_type) in &out.polygons {
        diagram = diagram + draw_poly(vs, poly_color(*poly_type));
    }

    // White edges on top.
    for &(v1, v2) in &out.edges {
        diagram = diagram + draw_edge(v1, v2);
    }

    let opts = RenderOptions {
        padding: 0.5,
        background: Some(Color::rgb(0.0, 0.0, 0.0)),
        default_stroke_width: THIN,
    };

    let svg = render_svg(&diagram, &opts);
    fs::write("tiling.svg", &svg).expect("failed to write tiling.svg");
    println!("Wrote tiling.svg ({} bytes)", svg.len());
}
