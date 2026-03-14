//! SVG rendering backend.
//!
//! Walks the [`DiagramNode`] scene tree, accumulates style and affine
//! transforms, and emits SVG text.

use std::fmt::Write as FmtWrite;

use kurbo::{Affine, CubicBez, Point};

use crate::backend::{Backend, RenderOptions};
use crate::diagram::{Diagram, DiagramNode};
use crate::style::{Color, RadialGradient, Style};

/// The SVG backend. Renders a [`Diagram`] to an SVG string.
pub struct SvgBackend;

impl SvgBackend {
    pub fn new() -> Self { SvgBackend }
}

impl Default for SvgBackend {
    fn default() -> Self { SvgBackend::new() }
}

// ── Error type ────────────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct SvgError(String);

impl std::fmt::Display for SvgError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SVG render error: {}", self.0)
    }
}

impl std::error::Error for SvgError {}

// ── Backend implementation ────────────────────────────────────────────────────

impl Backend for SvgBackend {
    type Output = String;
    type Error = SvgError;

    fn render(&mut self, diagram: &Diagram, opts: &RenderOptions) -> Result<String, SvgError> {
        let (vx, vy, vw, vh) = match diagram.bbox.rect() {
            None => (0.0_f64, 0.0_f64, 100.0_f64, 100.0_f64),
            Some(r) => {
                let pad = opts.padding;
                (r.x0 - pad, r.y0 - pad, r.width() + 2.0 * pad, r.height() + 2.0 * pad)
            }
        };

        let mut svg = String::new();
        writeln!(
            svg,
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="{vx:.4} {vy:.4} {vw:.4} {vh:.4}">"#
        )
        .unwrap();

        if let Some(bg) = opts.background {
            writeln!(
                svg,
                r#"  <rect x="{vx:.4}" y="{vy:.4}" width="{vw:.4}" height="{vh:.4}" fill="{}" />"#,
                bg.to_svg_color()
            )
            .unwrap();
        }

        // Resolve the default stroke width against the diagram's larger dimension.
        let diagram_size = match diagram.bbox.rect() {
            None => 1.0,
            Some(r) => r.width().max(r.height()),
        };
        let default_style = Style {
            stroke_color: Some(Color::rgb(0.0, 0.0, 0.0)),
            stroke_width: Some(opts.default_stroke_width.resolve(diagram_size)),
            ..Default::default()
        };

        let mut defs = String::new();
        let mut body = String::new();
        let mut grad_count = 0usize;
        render_node(
            &diagram.node, Affine::IDENTITY, &default_style,
            None, &mut defs, &mut grad_count, &mut body,
        );

        if !defs.is_empty() {
            writeln!(svg, "  <defs>").unwrap();
            svg.push_str(&defs);
            writeln!(svg, "  </defs>").unwrap();
        }
        svg.push_str(&body);
        writeln!(svg, "</svg>").unwrap();
        Ok(svg)
    }
}

// ── Internal rendering ────────────────────────────────────────────────────────

/// `grad_id`: ID of the currently active radial gradient fill (if any).
fn render_node(
    node: &DiagramNode,
    xf: Affine,
    style: &Style,
    grad_id: Option<&str>,
    defs: &mut String,
    grad_count: &mut usize,
    body: &mut String,
) {
    match node {
        DiagramNode::Empty => {}

        DiagramNode::StrokedTrail { trail, start } => {
            if trail.is_empty() {
                return;
            }
            let pts: Vec<Point> = trail.to_points(*start).into_iter().map(|p| xf * p).collect();
            emit_polyline(&pts, style, grad_id, body);
        }

        DiagramNode::Circle { radius } => {
            let center = xf * Point::ORIGIN;
            let scale = affine_scale(xf);
            emit_circle(center, radius * scale, style, grad_id, body);
        }

        DiagramNode::Rect { width, height } => {
            let (hw, hh) = (width / 2.0, height / 2.0);
            let corners = [
                xf * Point::new(-hw, -hh),
                xf * Point::new(hw, -hh),
                xf * Point::new(hw, hh),
                xf * Point::new(-hw, hh),
            ];
            emit_polygon(&corners, style, grad_id, body);
        }

        DiagramNode::Polygon(pts) => {
            let transformed: Vec<Point> = pts.iter().map(|&p| xf * p).collect();
            emit_polygon(&transformed, style, grad_id, body);
        }

        DiagramNode::CubicPath { segments } => {
            let transformed: Vec<CubicBez> = segments.iter().map(|&s| xf * s).collect();
            emit_cubic_path(&transformed, style, grad_id, body);
        }

        DiagramNode::Text { content, font_size } => {
            let origin = xf * Point::ORIGIN;
            let scale = affine_scale(xf);
            emit_text(content, origin, font_size * scale, style, body);
        }

        DiagramNode::Styled { style: inner, child } => {
            let resolved = style.merge_over(inner);
            // Only emit a gradient def when THIS node's own style introduces one.
            // Inherited gradient IDs continue to flow via `grad_id`.
            if let Some(ref g) = inner.fill_gradient {
                let id = format!("rg{}", *grad_count);
                *grad_count += 1;
                emit_gradient_def(&id, g, defs);
                render_node(child, xf, &resolved, Some(&id), defs, grad_count, body);
            } else {
                render_node(child, xf, &resolved, grad_id, defs, grad_count, body);
            }
        }

        DiagramNode::Transformed { affine, child } => {
            render_node(child, xf * *affine, style, grad_id, defs, grad_count, body);
        }

        DiagramNode::Group(children) => {
            for child in children {
                render_node(child, xf, style, grad_id, defs, grad_count, body);
            }
        }
    }
}

/// Approximate uniform scale factor of an affine (from the first column).
fn affine_scale(aff: Affine) -> f64 {
    let c = aff.as_coeffs();
    (c[0] * c[0] + c[1] * c[1]).sqrt()
}

// ── SVG element emitters ──────────────────────────────────────────────────────

fn fill_attr(style: &Style, grad_id: Option<&str>) -> String {
    if let Some(id) = grad_id {
        format!("url(#{id})")
    } else {
        style.fill_color.map(|c| c.to_svg_color()).unwrap_or_else(|| "none".to_string())
    }
}

fn style_attrs(style: &Style, grad_id: Option<&str>) -> String {
    let fill = fill_attr(style, grad_id);
    let stroke = style
        .stroke_color
        .map(|c| c.to_svg_color())
        .unwrap_or_else(|| "none".to_string());
    let sw = style.stroke_width.unwrap_or(1.0);

    let mut s = format!(r#"fill="{fill}" stroke="{stroke}" stroke-width="{sw:.6}""#);

    if let Some(op) = style.opacity {
        write!(s, r#" opacity="{op:.6}""#).unwrap();
    }
    if let Some(ref dash) = style.dash {
        let da: Vec<String> = dash.dashes.iter().map(|d| format!("{d:.6}")).collect();
        let offset = dash.offset;
        write!(s, r#" stroke-dasharray="{}" stroke-dashoffset="{offset:.6}""#, da.join(" "))
            .unwrap();
    }
    s
}

fn emit_gradient_def(id: &str, g: &RadialGradient, defs: &mut String) {
    writeln!(
        defs,
        r#"    <radialGradient id="{id}" gradientUnits="userSpaceOnUse" cx="{:.4}" cy="{:.4}" r="{:.4}">"#,
        g.cx, g.cy, g.r
    ).unwrap();
    for stop in &g.stops {
        writeln!(
            defs,
            r#"      <stop offset="{:.4}" stop-color="{}" stop-opacity="{:.4}" />"#,
            stop.offset,
            stop.color.to_svg_color(),
            stop.opacity,
        ).unwrap();
    }
    writeln!(defs, "    </radialGradient>").unwrap();
}

fn emit_polyline(pts: &[Point], style: &Style, grad_id: Option<&str>, svg: &mut String) {
    let mut points_attr = String::new();
    for (i, pt) in pts.iter().enumerate() {
        if i > 0 { points_attr.push(' '); }
        write!(points_attr, "{:.6},{:.6}", pt.x, pt.y).unwrap();
    }
    writeln!(svg, r#"  <polyline points="{points_attr}" {} />"#, style_attrs(style, grad_id)).unwrap();
}

fn emit_circle(center: Point, radius: f64, style: &Style, grad_id: Option<&str>, svg: &mut String) {
    writeln!(
        svg,
        r#"  <circle cx="{:.6}" cy="{:.6}" r="{:.6}" {} />"#,
        center.x, center.y, radius,
        style_attrs(style, grad_id)
    )
    .unwrap();
}

fn emit_polygon(pts: &[Point], style: &Style, grad_id: Option<&str>, svg: &mut String) {
    let mut points_attr = String::new();
    for (i, pt) in pts.iter().enumerate() {
        if i > 0 { points_attr.push(' '); }
        write!(points_attr, "{:.6},{:.6}", pt.x, pt.y).unwrap();
    }
    writeln!(svg, r#"  <polygon points="{points_attr}" {} />"#, style_attrs(style, grad_id)).unwrap();
}

fn emit_text(content: &str, origin: Point, font_size: f64, style: &Style, svg: &mut String) {
    let fill = style
        .fill_color
        .map(|c| c.to_svg_color())
        .unwrap_or_else(|| "black".to_string());
    let font_weight = if style.bold.unwrap_or(false) { "bold" } else { "normal" };
    let mut attrs = format!(
        r#"x="{:.6}" y="{:.6}" font-size="{:.6}" fill="{fill}" font-weight="{font_weight}" text-anchor="middle" dominant-baseline="central""#,
        origin.x, origin.y, font_size
    );
    if let Some(ref ff) = style.font_family {
        write!(attrs, r#" font-family="{ff}""#).unwrap();
    }
    if let Some(op) = style.opacity {
        write!(attrs, r#" opacity="{op:.6}""#).unwrap();
    }
    writeln!(svg, r#"  <text {attrs}>{}</text>"#, escape_xml(content)).unwrap();
}

fn emit_cubic_path(segments: &[CubicBez], style: &Style, grad_id: Option<&str>, svg: &mut String) {
    if segments.is_empty() {
        return;
    }
    let mut d = String::new();
    let p0 = segments[0].p0;
    write!(d, "M {:.6},{:.6}", p0.x, p0.y).unwrap();
    for seg in segments {
        write!(
            d, " C {:.6},{:.6} {:.6},{:.6} {:.6},{:.6}",
            seg.p1.x, seg.p1.y,
            seg.p2.x, seg.p2.y,
            seg.p3.x, seg.p3.y,
        )
        .unwrap();
    }
    writeln!(svg, r#"  <path d="{d}" {} />"#, style_attrs(style, grad_id)).unwrap();
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

// ── Tests ─────────────────────────���───────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagram::stroke_trail;
    use crate::primitives::{circle, rect, text};
    use crate::style::SILVER;
    use crate::trail::hrule;

    fn render(d: &Diagram) -> String {
        SvgBackend::new().render(d, &RenderOptions::default()).unwrap()
    }

    #[test]
    fn renders_stroked_trail() {
        let d = stroke_trail(hrule(10.0)).lc(SILVER);
        let svg = render(&d);
        assert!(svg.contains("<polyline"), "expected polyline in: {svg}");
    }

    #[test]
    fn renders_circle() {
        let svg = render(&circle(1.0).fc(crate::style::BLUE));
        assert!(svg.contains("<circle"), "expected circle in: {svg}");
    }

    #[test]
    fn renders_rect() {
        let svg = render(&rect(2.0, 1.0).fc(crate::style::RED));
        assert!(svg.contains("<polygon"), "expected polygon in: {svg}");
    }

    #[test]
    fn renders_text() {
        let svg = render(&text("hello", 0.5));
        assert!(svg.contains("<text"), "expected text in: {svg}");
        assert!(svg.contains("hello"), "expected content in: {svg}");
    }

    #[test]
    fn renders_dashed_line() {
        let d = stroke_trail(hrule(5.0)).dashing(vec![0.1, 0.05], 0.0);
        let svg = render(&d);
        assert!(svg.contains("stroke-dasharray"), "expected dasharray in: {svg}");
    }

    #[test]
    fn svg_has_viewbox_from_bbox() {
        let d = circle(5.0);
        let svg = render(&d);
        assert!(svg.contains("viewBox="));
    }
}
