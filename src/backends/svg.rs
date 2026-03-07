//! SVG rendering backend.
//!
//! Walks the [`DiagramNode`] scene tree, accumulates style, and emits SVG.

use std::fmt::Write as FmtWrite;

use kurbo::Point;

use crate::backend::{Backend, RenderOptions};
use crate::diagram::{Diagram, DiagramNode};
use crate::style::{Color, Style};

/// The SVG backend. Renders a [`Diagram`] to an SVG string.
pub struct SvgBackend;

impl SvgBackend {
    pub fn new() -> Self {
        SvgBackend
    }
}

impl Default for SvgBackend {
    fn default() -> Self {
        SvgBackend::new()
    }
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
        // First pass: collect all path points to compute the bounding box.
        let mut all_points: Vec<Point> = Vec::new();
        collect_points(&diagram.node, &mut all_points);

        let (vx, vy, vw, vh) = if all_points.is_empty() {
            (0.0_f64, 0.0_f64, 100.0_f64, 100.0_f64)
        } else {
            let min_x = all_points.iter().map(|p| p.x).fold(f64::INFINITY, f64::min);
            let min_y = all_points.iter().map(|p| p.y).fold(f64::INFINITY, f64::min);
            let max_x = all_points.iter().map(|p| p.x).fold(f64::NEG_INFINITY, f64::max);
            let max_y = all_points.iter().map(|p| p.y).fold(f64::NEG_INFINITY, f64::max);
            let pad = opts.padding;
            (min_x - pad, min_y - pad, (max_x - min_x) + 2.0 * pad, (max_y - min_y) + 2.0 * pad)
        };

        let mut svg = String::new();

        writeln!(
            svg,
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="{vx:.4} {vy:.4} {vw:.4} {vh:.4}">"#,
        )
        .unwrap();

        // Optional background rect.
        if let Some(bg) = opts.background {
            writeln!(
                svg,
                r#"  <rect x="{vx:.4}" y="{vy:.4}" width="{vw:.4}" height="{vh:.4}" fill="{}" />"#,
                bg.to_svg_color(),
            )
            .unwrap();
        }

        // Second pass: emit SVG elements.
        let default_style = Style {
            stroke_color: Some(Color::rgb(0.0, 0.0, 0.0)),
            stroke_width: Some(opts.default_stroke_width),
            fill_color: None,
            opacity: None,
        };
        render_node(&diagram.node, &default_style, &mut svg);

        writeln!(svg, "</svg>").unwrap();

        Ok(svg)
    }
}

// ── Internal helpers ──────────────────────────────────────────────────────────

/// Recursively collect all trail points for bounding-box computation.
fn collect_points(node: &DiagramNode, out: &mut Vec<Point>) {
    match node {
        DiagramNode::Empty => {}
        DiagramNode::StrokedTrail { trail, start } => {
            out.extend(trail.to_points(*start));
        }
        DiagramNode::Styled { child, .. } => {
            collect_points(child, out);
        }
        DiagramNode::Group(children) => {
            for child in children {
                collect_points(child, out);
            }
        }
    }
}

/// Recursively render nodes into the SVG string, accumulating style.
fn render_node(node: &DiagramNode, inherited: &Style, svg: &mut String) {
    match node {
        DiagramNode::Empty => {}

        DiagramNode::StrokedTrail { trail, start } => {
            if trail.is_empty() {
                return;
            }
            let pts = trail.to_points(*start);
            emit_polyline(&pts, inherited, svg);
        }

        DiagramNode::Styled { style, child } => {
            // Inner style (this node) wins over inherited (outer).
            let resolved = inherited.merge_over(style);
            render_node(child, &resolved, svg);
        }

        DiagramNode::Group(children) => {
            for child in children {
                render_node(child, inherited, svg);
            }
        }
    }
}

/// Emit a `<polyline>` SVG element.
fn emit_polyline(pts: &[Point], style: &Style, svg: &mut String) {
    // Build the points attribute.
    let mut points_attr = String::new();
    for (i, pt) in pts.iter().enumerate() {
        if i > 0 {
            points_attr.push(' ');
        }
        write!(points_attr, "{:.6},{:.6}", pt.x, pt.y).unwrap();
    }

    let fill = style.fill_color.map(|c| c.to_svg_color()).unwrap_or_else(|| "none".to_string());
    let stroke = style.stroke_color.map(|c| c.to_svg_color()).unwrap_or_else(|| "none".to_string());
    let stroke_width = style.stroke_width.unwrap_or(1.0);

    let mut attrs = format!(
        r#"fill="{fill}" stroke="{stroke}" stroke-width="{stroke_width:.6}""#,
    );

    if let Some(op) = style.opacity {
        write!(attrs, r#" opacity="{op:.6}""#).unwrap();
    }

    writeln!(svg, r#"  <polyline points="{points_attr}" {attrs} />"#).unwrap();
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagram::stroke_trail;
    use crate::style::SILVER;
    use crate::trail::hrule;

    fn render(d: &Diagram) -> String {
        SvgBackend::new().render(d, &RenderOptions::default()).unwrap()
    }

    #[test]
    fn renders_simple_line_to_svg() {
        let d = stroke_trail(hrule(10.0)).lc(SILVER);
        let svg = render(&d);
        assert!(svg.contains("<svg"));
        assert!(svg.contains("<polyline"));
        assert!(svg.contains("</svg>"));
    }

    #[test]
    fn svg_contains_viewbox() {
        let d = stroke_trail(hrule(10.0));
        let svg = render(&d);
        assert!(svg.contains("viewBox="));
    }
}
