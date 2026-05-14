//! Integration tests for hagoromo's Gluon bindings.

use gluon::vm::api::UserdataValue;
use gluon::{new_vm, ThreadExt};

use hagoromo::gluon::userdata::{GBoundingBox, GColor, GDiagram};

fn vm() -> gluon::RootedThread {
    let vm = new_vm();
    // Disable the implicit prelude to avoid a gluon 0.18.2 bug where
    // `stringify!` on `std::io::File` produces `" File"` (space before F)
    // on modern Rust, causing a type-registration assertion failure.
    // Hagoromo scripts don't need the gluon stdlib.
    vm.get_database_mut().set_implicit_prelude(false);
    hagoromo::gluon::register(&vm).unwrap();
    vm.load_script("hagoromo", hagoromo::gluon::PRELUDE).unwrap();
    vm
}

#[test]
fn color_round_trip() {
    let vm = vm();
    let src = r#"
        let { prim } = import! hagoromo
        prim.color.red
    "#;
    let (UserdataValue(c), _) = vm
        .run_expr::<UserdataValue<GColor>>("color_round_trip", src)
        .unwrap();
    assert_eq!(c.0, hagoromo::style::RED);
}

#[test]
fn circle_with_fill_has_unit_bbox() {
    let vm = vm();
    let src = r#"
        let { prim, (|>) } = import! hagoromo
        let { circle, color, fc } = prim
        circle 1.0 |> fc color.red
    "#;
    let (UserdataValue(d), _) = vm
        .run_expr::<UserdataValue<GDiagram>>("circle_fc", src)
        .unwrap();
    let r = d.0.bbox().rect().expect("bbox should exist");
    assert!((r.width() - 2.0).abs() < 1e-9);
    assert!((r.height() - 2.0).abs() < 1e-9);
}

#[test]
fn hcat_three_circles_has_width_six() {
    let vm = vm();
    let src = r#"
        let { prim } = import! hagoromo
        let { circle, hcat } = prim
        hcat [circle 1.0, circle 1.0, circle 1.0]
    "#;
    let (UserdataValue(d), _) = vm
        .run_expr::<UserdataValue<GDiagram>>("hcat_three", src)
        .unwrap();
    let r = d.0.bbox().rect().unwrap();
    assert!((r.width() - 6.0).abs() < 1e-9);
}

#[test]
fn render_svg_produces_expected_markup() {
    let vm = vm();
    let src = r#"
        let { prim, (|>) } = import! hagoromo
        let { circle, color, fc, render_svg, render_options_default } = prim
        render_svg (circle 1.0 |> fc color.red) render_options_default
    "#;
    let (svg, _) = vm.run_expr::<String>("svg_render", src).unwrap();
    assert!(svg.contains("<svg"), "expected <svg in output, got: {svg}");
    assert!(svg.contains("<circle"), "expected <circle in output");
    assert!(
        svg.contains("#ff0000"),
        "expected red fill (#ff0000) in output, got: {svg}"
    );
}

#[test]
fn operator_precedence_atop_and_chain() {
    let vm = vm();
    // |> at precedence 8, <> at 6 — so `d1 |> f <> d2 |> g`
    // parses as `(d1 |> f) <> (d2 |> g)`.
    let src = r#"
        let { prim, (<>), (|>) } = import! hagoromo
        let { circle, color, fc } = prim
        circle 1.0 |> fc color.red <> circle 2.0 |> fc color.blue
    "#;
    let (UserdataValue(d), _) = vm
        .run_expr::<UserdataValue<GDiagram>>("op_prec", src)
        .unwrap();
    // Two superimposed circles (radii 1 and 2) — bbox should be 4×4.
    let r = d.0.bbox().rect().unwrap();
    assert!((r.width() - 4.0).abs() < 1e-9);
    assert!((r.height() - 4.0).abs() < 1e-9);
}

#[test]
fn support_in_returns_some_for_circle() {
    let vm = vm();
    let src = r#"
        let { prim } = import! hagoromo
        let { circle, support_in, direction } = prim
        support_in direction.right (circle 1.0)
    "#;
    let (val, _) = vm.run_expr::<Option<f64>>("support", src).unwrap();
    let v = val.expect("support_in should return Some for a unit circle");
    assert!((v - 1.0).abs() < 1e-9);
}

#[test]
fn beside_operators_layout_horizontally_and_vertically() {
    let vm = vm();
    let (UserdataValue(h), _) = vm
        .run_expr::<UserdataValue<GDiagram>>(
            "lay_h",
            r#"
            let { prim, (|||) } = import! hagoromo
            prim.circle 1.0 ||| prim.circle 1.0
        "#,
        )
        .unwrap();
    let (UserdataValue(v), _) = vm
        .run_expr::<UserdataValue<GDiagram>>(
            "lay_v",
            r#"
            let { prim, (===) } = import! hagoromo
            prim.circle 1.0 === prim.circle 1.0
        "#,
        )
        .unwrap();
    let rh = h.0.bbox().rect().unwrap();
    let rv = v.0.bbox().rect().unwrap();
    assert!((rh.width() - 4.0).abs() < 1e-9, "h width = {}", rh.width());
    assert!((rv.height() - 4.0).abs() < 1e-9, "v height = {}", rv.height());
}

#[test]
fn bbox_query_through_script() {
    let vm = vm();
    let src = r#"
        let { prim } = import! hagoromo
        let { circle, bbox } = prim
        bbox (circle 3.0)
    "#;
    let (UserdataValue(b), _) = vm
        .run_expr::<UserdataValue<GBoundingBox>>("bbox_q", src)
        .unwrap();
    let r = b.0.rect().unwrap();
    assert!((r.width() - 6.0).abs() < 1e-9);
}
