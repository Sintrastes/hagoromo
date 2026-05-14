//! All hagoromo primitives exposed to Gluon scripts.
//!
//! Functions follow the convention that the "receiver" diagram/value is the
//! **last** parameter, so scripts can use the prelude's `|>` operator
//! (`circle 1.0 |> fc red |> lw 0.5`).
//!
//! Userdata is received either as `&G*` (single argument, cheap clone) or
//! as `UserdataValue<G*>` (when wrapped in `Vec`/`Option`/tuple, where a
//! reference would carry an awkward lifetime). Hagoromo's types are all
//! `Arc`-backed or pure value types, so cloning is cheap.

#![allow(clippy::too_many_arguments)]

use std::sync::Arc;

use gluon::vm::{self, ExternModule};
use gluon::Thread;
use gluon_vm::api::UserdataValue;
use gluon_vm::{primitive, record};
use kurbo::{Affine, Point, Rect, Vec2};

use crate as hag;
use crate::gluon::userdata::{
    GBoundingBox, GColor, GCubicSpline, GDiagram, GGradientStop, GMeasure, GPoint, GRadialGradient,
    GRect, GRenderOptions, GTrail, GVec2,
};

// ── Geometry helpers (Point / Vec2 / Rect) ───────────────────────────────────

fn point_(x: f64, y: f64) -> GPoint { GPoint(Point::new(x, y)) }
fn point_x(p: &GPoint) -> f64 { p.0.x }
fn point_y(p: &GPoint) -> f64 { p.0.y }

fn vec2_(x: f64, y: f64) -> GVec2 { GVec2(Vec2::new(x, y)) }
fn vec2_x(v: &GVec2) -> f64 { v.0.x }
fn vec2_y(v: &GVec2) -> f64 { v.0.y }
fn vec2_length(v: &GVec2) -> f64 { v.0.length() }

fn rect_new(x0: f64, y0: f64, x1: f64, y1: f64) -> GRect {
    GRect(Rect::new(x0, y0, x1, y1))
}
fn rect_x0(r: &GRect) -> f64 { r.0.x0 }
fn rect_y0(r: &GRect) -> f64 { r.0.y0 }
fn rect_x1(r: &GRect) -> f64 { r.0.x1 }
fn rect_y1(r: &GRect) -> f64 { r.0.y1 }
fn rect_width(r: &GRect) -> f64 { r.0.width() }
fn rect_height(r: &GRect) -> f64 { r.0.height() }

// ── Color ────────────────────────────────────────────────────────────────────

fn color_rgb(r: f64, g: f64, b: f64) -> GColor {
    GColor(hag::Color::rgb(r as f32, g as f32, b as f32))
}
fn color_rgba(r: f64, g: f64, b: f64, a: f64) -> GColor {
    GColor(hag::Color::rgba(r as f32, g as f32, b as f32, a as f32))
}
fn color_rgb_bytes(r: i32, g: i32, b: i32) -> GColor {
    GColor(hag::Color::rgb_bytes(r as u8, g as u8, b as u8))
}
fn color_rgba_bytes(r: i32, g: i32, b: i32, a: i32) -> GColor {
    // Workaround: hagoromo's Color::rgba_bytes ignores its `a` argument and
    // hardcodes 1.0. Compute alpha correctly here.
    GColor(hag::Color::rgba(
        (r as f32) / 255.0,
        (g as f32) / 255.0,
        (b as f32) / 255.0,
        (a as f32) / 255.0,
    ))
}
fn color_from_hex(hex: String) -> GColor { GColor(hag::Color::from_hex(&hex)) }
fn color_to_svg(c: &GColor) -> String { c.0.to_svg_color() }
fn color_to_hex(c: &GColor) -> String { c.0.to_hex() }
fn color_alpha(c: &GColor) -> f64 { c.0.alpha() as f64 }

// ── Measure ──────────────────────────────────────────────────────────────────

fn measure_absolute(w: f64) -> GMeasure { GMeasure(hag::Measure::Absolute(w)) }
fn measure_normalized(f: f64) -> GMeasure { GMeasure(hag::Measure::Normalized(f)) }

// ── Gradient ─────────────────────────────────────────────────────────────────

fn gradient_stop(offset: f64, color: &GColor, opacity: f64) -> GGradientStop {
    GGradientStop(hag::GradientStop { offset, color: color.0, opacity })
}
fn radial_gradient(r: f64, stops: Vec<UserdataValue<GGradientStop>>) -> GRadialGradient {
    GRadialGradient(hag::RadialGradient::new(
        r,
        stops.into_iter().map(|UserdataValue(s)| s.0).collect(),
    ))
}

// ── Trail ────────────────────────────────────────────────────────────────────

fn trail_empty() -> GTrail { GTrail(hag::Trail::empty()) }
fn hrule(length: f64) -> GTrail { GTrail(hag::hrule(length)) }
fn vrule(length: f64) -> GTrail { GTrail(hag::vrule(length)) }
fn trail_concat(a: &GTrail, b: &GTrail) -> GTrail { GTrail(a.0.clone() + b.0.clone()) }
fn trail_reflect_x(t: &GTrail) -> GTrail { GTrail(t.0.clone().reflect_x()) }
fn trail_reflect_y(t: &GTrail) -> GTrail { GTrail(t.0.clone().reflect_y()) }
fn trail_rotate_by(turns: f64, t: &GTrail) -> GTrail { GTrail(t.0.clone().rotate_by(turns)) }
fn trail_total_displacement(t: &GTrail) -> GVec2 { GVec2(t.0.total_displacement()) }
fn trail_to_points(start: &GPoint, t: &GTrail) -> Vec<GPoint> {
    t.0.to_points(start.0).into_iter().map(GPoint).collect()
}
fn trail_len(t: &GTrail) -> i32 { t.0.len() as i32 }
fn trail_is_empty(t: &GTrail) -> bool { t.0.is_empty() }

// ── Primitives ───────────────────────────────────────────────────────────────

fn diagram_empty() -> GDiagram { GDiagram(hag::Diagram::empty()) }
fn circle(r: f64) -> GDiagram { GDiagram(hag::circle(r)) }
fn rect_(w: f64, h: f64) -> GDiagram { GDiagram(hag::rect(w, h)) }
fn square(s: f64) -> GDiagram { GDiagram(hag::square(s)) }
fn equilateral_triangle(s: f64) -> GDiagram { GDiagram(hag::equilateral_triangle(s)) }
fn polygon_(pts: Vec<UserdataValue<GPoint>>) -> GDiagram {
    let pts: Vec<Point> = pts.into_iter().map(|UserdataValue(p)| p.0).collect();
    GDiagram(hag::polygon(&pts))
}
fn reg_poly(sides: i32, side_len: f64) -> GDiagram {
    GDiagram(hag::reg_poly(sides as usize, side_len))
}
fn polyline_(pts: Vec<UserdataValue<GPoint>>) -> GDiagram {
    let pts: Vec<Point> = pts.into_iter().map(|UserdataValue(p)| p.0).collect();
    GDiagram(hag::polyline(&pts))
}
fn text_(content: String, font_size: f64) -> GDiagram { GDiagram(hag::text(content, font_size)) }
fn strut_x(w: f64) -> GDiagram { GDiagram(hag::strut_x(w)) }
fn strut_y(h: f64) -> GDiagram { GDiagram(hag::strut_y(h)) }
fn stroke_trail(t: &GTrail) -> GDiagram { GDiagram(hag::stroke_trail(t.0.clone())) }
fn stroke_spline(s: &GCubicSpline) -> GDiagram { GDiagram(hag::stroke_spline(&s.0)) }

// ── Diagram styling (receiver-last for `#` operator) ─────────────────────────

fn lc(c: &GColor, d: &GDiagram) -> GDiagram { GDiagram(d.0.clone().lc(c.0)) }
fn fc(c: &GColor, d: &GDiagram) -> GDiagram { GDiagram(d.0.clone().fc(c.0)) }
fn lw(w: f64, d: &GDiagram) -> GDiagram { GDiagram(d.0.clone().lw(w)) }
fn opacity(o: f64, d: &GDiagram) -> GDiagram { GDiagram(d.0.clone().opacity(o)) }
fn dashing(dashes: Vec<f64>, offset: f64, d: &GDiagram) -> GDiagram {
    GDiagram(d.0.clone().dashing(dashes, offset))
}
fn fill_gradient(grad: &GRadialGradient, d: &GDiagram) -> GDiagram {
    GDiagram(d.0.clone().fill_gradient(grad.0.clone()))
}
fn bold(d: &GDiagram) -> GDiagram { GDiagram(d.0.clone().bold()) }
fn font_family(family: String, d: &GDiagram) -> GDiagram {
    GDiagram(d.0.clone().font_family(family))
}
fn bg(c: &GColor, d: &GDiagram) -> GDiagram { GDiagram(d.0.clone().bg(c.0)) }

// ── Diagram transforms ───────────────────────────────────────────────────────

fn translate(dx: f64, dy: f64, d: &GDiagram) -> GDiagram { GDiagram(d.0.clone().translate(dx, dy)) }
fn translate_x(dx: f64, d: &GDiagram) -> GDiagram { GDiagram(d.0.clone().translate_x(dx)) }
fn translate_y(dy: f64, d: &GDiagram) -> GDiagram { GDiagram(d.0.clone().translate_y(dy)) }
fn scale(s: f64, d: &GDiagram) -> GDiagram { GDiagram(d.0.clone().scale(s)) }
fn scale_xy(sx: f64, sy: f64, d: &GDiagram) -> GDiagram { GDiagram(d.0.clone().scale_xy(sx, sy)) }
fn rotate(angle: f64, d: &GDiagram) -> GDiagram { GDiagram(d.0.clone().rotate(angle)) }
fn rotate_by(turns: f64, d: &GDiagram) -> GDiagram { GDiagram(d.0.clone().rotate_by(turns)) }
fn reflect_x_d(d: &GDiagram) -> GDiagram { GDiagram(d.0.clone().reflect_x()) }
fn reflect_y_d(d: &GDiagram) -> GDiagram { GDiagram(d.0.clone().reflect_y()) }
fn align_left(d: &GDiagram) -> GDiagram { GDiagram(d.0.clone().align_left()) }
fn align_right(d: &GDiagram) -> GDiagram { GDiagram(d.0.clone().align_right()) }
fn align_top(d: &GDiagram) -> GDiagram { GDiagram(d.0.clone().align_top()) }
fn align_bottom(d: &GDiagram) -> GDiagram { GDiagram(d.0.clone().align_bottom()) }
fn center_x(d: &GDiagram) -> GDiagram { GDiagram(d.0.clone().center_x()) }
fn center_y(d: &GDiagram) -> GDiagram { GDiagram(d.0.clone().center_y()) }

// ── Diagram queries ──────────────────────────────────────────────────────────

fn bbox_of(d: &GDiagram) -> GBoundingBox { GBoundingBox(d.0.bbox()) }
fn support_in(dir: &GVec2, d: &GDiagram) -> Option<f64> { d.0.support_in(dir.0) }

// ── Layout combinators ───────────────────────────────────────────────────────

fn atop(a: &GDiagram, b: &GDiagram) -> GDiagram { GDiagram(hag::atop(a.0.clone(), b.0.clone())) }
fn beside_right(a: &GDiagram, b: &GDiagram) -> GDiagram {
    GDiagram(hag::beside(hag::RIGHT, a.0.clone(), b.0.clone()))
}
fn beside_down(a: &GDiagram, b: &GDiagram) -> GDiagram {
    GDiagram(hag::beside(hag::DOWN, a.0.clone(), b.0.clone()))
}
fn beside_(dir: &GVec2, a: &GDiagram, b: &GDiagram) -> GDiagram {
    GDiagram(hag::beside(dir.0, a.0.clone(), b.0.clone()))
}
fn hcat(items: Vec<UserdataValue<GDiagram>>) -> GDiagram {
    GDiagram(hag::hcat(items.into_iter().map(|UserdataValue(g)| g.0)))
}
fn vcat(items: Vec<UserdataValue<GDiagram>>) -> GDiagram {
    GDiagram(hag::vcat(items.into_iter().map(|UserdataValue(g)| g.0)))
}
fn hcat_sep(sep: f64, items: Vec<UserdataValue<GDiagram>>) -> GDiagram {
    GDiagram(hag::hcat_sep(sep, items.into_iter().map(|UserdataValue(g)| g.0)))
}
fn vcat_sep(sep: f64, items: Vec<UserdataValue<GDiagram>>) -> GDiagram {
    GDiagram(hag::vcat_sep(sep, items.into_iter().map(|UserdataValue(g)| g.0)))
}
fn position_(items: Vec<(UserdataValue<GPoint>, UserdataValue<GDiagram>)>) -> GDiagram {
    GDiagram(hag::position(
        items.into_iter().map(|(UserdataValue(p), UserdataValue(d))| (p.0, d.0)),
    ))
}
fn appends(base: &GDiagram, items: Vec<(UserdataValue<GVec2>, UserdataValue<GDiagram>)>) -> GDiagram {
    GDiagram(hag::appends(
        base.0.clone(),
        items.into_iter().map(|(UserdataValue(v), UserdataValue(d))| (v.0, d.0)),
    ))
}

// ── BoundingBox ──────────────────────────────────────────────────────────────

fn bbox_empty() -> GBoundingBox { GBoundingBox(hag::BoundingBox::EMPTY) }
fn bbox_from_rect(r: &GRect) -> GBoundingBox { GBoundingBox(hag::BoundingBox::from_rect(r.0)) }
fn bbox_from_points(pts: Vec<UserdataValue<GPoint>>) -> GBoundingBox {
    let pts: Vec<Point> = pts.into_iter().map(|UserdataValue(p)| p.0).collect();
    GBoundingBox(hag::BoundingBox::from_points(&pts))
}
fn bbox_rect(b: &GBoundingBox) -> Option<GRect> { b.0.rect().map(GRect) }
fn bbox_union(a: &GBoundingBox, b: &GBoundingBox) -> GBoundingBox { GBoundingBox(a.0.union(b.0)) }
fn bbox_translate(v: &GVec2, b: &GBoundingBox) -> GBoundingBox {
    GBoundingBox(b.0.translate(v.0))
}
fn bbox_scale(s: f64, b: &GBoundingBox) -> GBoundingBox {
    GBoundingBox(b.0.transform(Affine::scale(s)))
}
fn bbox_rotate(angle: f64, b: &GBoundingBox) -> GBoundingBox {
    GBoundingBox(b.0.transform(Affine::rotate(angle)))
}
fn extent_in(dir: &GVec2, b: &GBoundingBox) -> Option<f64> { b.0.extent_in(dir.0) }

// ── CubicSpline ──────────────────────────────────────────────────────────────

fn cubic_spline_(pts: Vec<UserdataValue<GPoint>>) -> GCubicSpline {
    let pts: Vec<Point> = pts.into_iter().map(|UserdataValue(p)| p.0).collect();
    GCubicSpline(Arc::new(hag::cubic_spline(&pts)))
}
fn at_param(t: f64, s: &GCubicSpline) -> GPoint { GPoint(s.0.at_param(t)) }
fn tangent_at_param(t: f64, s: &GCubicSpline) -> GVec2 { GVec2(s.0.tangent_at_param(t)) }
fn normal_at_param(t: f64, s: &GCubicSpline) -> GVec2 { GVec2(s.0.normal_at_param(t)) }

// ── Render ───────────────────────────────────────────────────────────────────

fn render_options_default() -> GRenderOptions { GRenderOptions(hag::RenderOptions::default()) }
fn render_options(
    padding: f64,
    background: Option<UserdataValue<GColor>>,
    default_stroke_width: &GMeasure,
) -> GRenderOptions {
    GRenderOptions(hag::RenderOptions {
        padding,
        background: background.map(|UserdataValue(c)| c.0),
        default_stroke_width: default_stroke_width.0,
    })
}
fn render_options_bg(padding: f64, bg: &GColor, default_stroke_width: &GMeasure) -> GRenderOptions {
    GRenderOptions(hag::RenderOptions {
        padding,
        background: Some(bg.0),
        default_stroke_width: default_stroke_width.0,
    })
}
fn render_svg(d: &GDiagram, opts: &GRenderOptions) -> String {
    hag::render_svg(&d.0, &opts.0)
}

// ── Module loader ────────────────────────────────────────────────────────────

pub fn load(thread: &Thread) -> vm::Result<ExternModule> {
    ExternModule::new(thread, record! {
        // Userdata types (so scripts can write type signatures)
        type Diagram        => GDiagram,
        type Trail          => GTrail,
        type Color          => GColor,
        type Measure        => GMeasure,
        type BoundingBox    => GBoundingBox,
        type Rect           => GRect,
        type Point          => GPoint,
        type Vec2           => GVec2,
        type CubicSpline    => GCubicSpline,
        type RadialGradient => GRadialGradient,
        type GradientStop   => GGradientStop,
        type RenderOptions  => GRenderOptions,

        // ── Geometry constructors / accessors ───────────────────────────
        point          => primitive!(2, point_),
        point_x        => primitive!(1, point_x),
        point_y        => primitive!(1, point_y),
        vec2           => primitive!(2, vec2_),
        vec2_x         => primitive!(1, vec2_x),
        vec2_y         => primitive!(1, vec2_y),
        vec2_length    => primitive!(1, vec2_length),
        rect_new       => primitive!(4, rect_new),
        rect_x0        => primitive!(1, rect_x0),
        rect_y0        => primitive!(1, rect_y0),
        rect_x1        => primitive!(1, rect_x1),
        rect_y1        => primitive!(1, rect_y1),
        rect_width     => primitive!(1, rect_width),
        rect_height    => primitive!(1, rect_height),

        // ── Color (constants + constructors grouped) ────────────────────
        color => record! {
            black       => GColor(hag::style::BLACK),
            white       => GColor(hag::style::WHITE),
            red         => GColor(hag::style::RED),
            green       => GColor(hag::style::GREEN),
            blue        => GColor(hag::style::BLUE),
            silver      => GColor(hag::style::SILVER),
            transparent => GColor(crate::style::TRANSPARENT),
            rgb         => primitive!(3, color_rgb),
            rgba        => primitive!(4, color_rgba),
            rgb_bytes   => primitive!(3, color_rgb_bytes),
            rgba_bytes  => primitive!(4, color_rgba_bytes),
            from_hex    => primitive!(1, color_from_hex),
            to_svg      => primitive!(1, color_to_svg),
            to_hex      => primitive!(1, color_to_hex),
            alpha       => primitive!(1, color_alpha)
        },

        // ── Measure (constants + constructors) ──────────────────────────
        measure => record! {
            none        => GMeasure(hag::style::NONE),
            ultra_thin  => GMeasure(hag::style::ULTRA_THIN),
            very_thin   => GMeasure(hag::style::VERY_THIN),
            thin        => GMeasure(hag::style::THIN),
            medium      => GMeasure(hag::style::MEDIUM),
            thick       => GMeasure(hag::style::THICK),
            very_thick  => GMeasure(hag::style::VERY_THICK),
            ultra_thick => GMeasure(hag::style::ULTRA_THICK),
            absolute    => primitive!(1, measure_absolute),
            normalized  => primitive!(1, measure_normalized)
        },

        // ── Direction constants ─────────────────────────────────────────
        direction => record! {
            right => GVec2(hag::RIGHT),
            left  => GVec2(hag::LEFT),
            up    => GVec2(hag::UP),
            down  => GVec2(hag::DOWN)
        },

        // ── Gradients ───────────────────────────────────────────────────
        gradient_stop      => primitive!(3, gradient_stop),
        radial_gradient    => primitive!(2, radial_gradient),

        // ── Trail ───────────────────────────────────────────────────────
        trail_empty             => GTrail(hag::Trail::empty()),
        hrule                   => primitive!(1, hrule),
        vrule                   => primitive!(1, vrule),
        trail_concat            => primitive!(2, trail_concat),
        trail_reflect_x         => primitive!(1, trail_reflect_x),
        trail_reflect_y         => primitive!(1, trail_reflect_y),
        trail_rotate_by         => primitive!(2, trail_rotate_by),
        trail_total_displacement=> primitive!(1, trail_total_displacement),
        trail_to_points         => primitive!(2, trail_to_points),
        trail_len               => primitive!(1, trail_len),
        trail_is_empty          => primitive!(1, trail_is_empty),

        // ── Primitives ──────────────────────────────────────────────────
        diagram_empty       => GDiagram(hag::Diagram::empty()),
        circle              => primitive!(1, circle),
        rect                => primitive!(2, rect_),
        square              => primitive!(1, square),
        equilateral_triangle=> primitive!(1, equilateral_triangle),
        polygon             => primitive!(1, polygon_),
        reg_poly            => primitive!(2, reg_poly),
        polyline            => primitive!(1, polyline_),
        text                => primitive!(2, text_),
        strut_x             => primitive!(1, strut_x),
        strut_y             => primitive!(1, strut_y),
        stroke_trail        => primitive!(1, stroke_trail),
        stroke_spline       => primitive!(1, stroke_spline),

        // ── Diagram styling (receiver-last) ─────────────────────────────
        lc              => primitive!(2, lc),
        stroke_color    => primitive!(2, lc),
        fc              => primitive!(2, fc),
        fill_color      => primitive!(2, fc),
        lw              => primitive!(2, lw),
        stroke_width    => primitive!(2, lw),
        opacity         => primitive!(2, opacity),
        dashing         => primitive!(3, dashing),
        fill_gradient   => primitive!(2, fill_gradient),
        bold            => primitive!(1, bold),
        font_family     => primitive!(2, font_family),
        bg              => primitive!(2, bg),

        // ── Diagram transforms ──────────────────────────────────────────
        translate    => primitive!(3, translate),
        translate_x  => primitive!(2, translate_x),
        translate_y  => primitive!(2, translate_y),
        scale        => primitive!(2, scale),
        scale_xy     => primitive!(3, scale_xy),
        rotate       => primitive!(2, rotate),
        rotate_by    => primitive!(2, rotate_by),
        reflect_x    => primitive!(1, reflect_x_d),
        reflect_y    => primitive!(1, reflect_y_d),
        align_left   => primitive!(1, align_left),
        align_right  => primitive!(1, align_right),
        align_top    => primitive!(1, align_top),
        align_bottom => primitive!(1, align_bottom),
        center_x     => primitive!(1, center_x),
        center_y     => primitive!(1, center_y),

        // ── Diagram queries ─────────────────────────────────────────────
        bbox        => primitive!(1, bbox_of),
        support_in  => primitive!(2, support_in),

        // ── Layout combinators ──────────────────────────────────────────
        atop          => primitive!(2, atop),
        beside_right  => primitive!(2, beside_right),
        beside_down   => primitive!(2, beside_down),
        beside        => primitive!(3, beside_),
        hcat          => primitive!(1, hcat),
        vcat          => primitive!(1, vcat),
        hcat_sep      => primitive!(2, hcat_sep),
        vcat_sep      => primitive!(2, vcat_sep),
        position      => primitive!(1, position_),
        appends       => primitive!(2, appends),

        // ── BoundingBox ─────────────────────────────────────────────────
        bbox_empty        => GBoundingBox(hag::BoundingBox::EMPTY),
        bbox_from_rect    => primitive!(1, bbox_from_rect),
        bbox_from_points  => primitive!(1, bbox_from_points),
        bbox_rect         => primitive!(1, bbox_rect),
        bbox_union        => primitive!(2, bbox_union),
        bbox_translate    => primitive!(2, bbox_translate),
        bbox_scale        => primitive!(2, bbox_scale),
        bbox_rotate       => primitive!(2, bbox_rotate),
        extent_in         => primitive!(2, extent_in),

        // ── CubicSpline ─────────────────────────────────────────────────
        cubic_spline      => primitive!(1, cubic_spline_),
        at_param          => primitive!(2, at_param),
        tangent_at_param  => primitive!(2, tangent_at_param),
        normal_at_param   => primitive!(2, normal_at_param),

        // ── Render ──────────────────────────────────────────────────────
        render_options          => primitive!(3, render_options),
        render_options_bg       => primitive!(3, render_options_bg),
        render_options_default  => GRenderOptions(hag::RenderOptions::default()),
        render_svg              => primitive!(2, render_svg)
    })
}
