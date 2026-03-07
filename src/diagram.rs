//! The [`Diagram`] type: the central composable drawing primitive.

use std::sync::Arc;

use kurbo::{Affine, Point, Vec2};

use crate::envelope::BoundingBox;
use crate::style::{Color, Style};
use crate::trail::Trail;

// ── Internal scene tree ───────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub(crate) enum DiagramNode {
    Empty,
    StrokedTrail { trail: Trail, start: Point },
    Circle { radius: f64 },
    Rect { width: f64, height: f64 },
    /// An arbitrary filled polygon. Vertices are in local coordinates.
    Polygon(Vec<Point>),
    /// Text centered at the origin. Font size in diagram units.
    Text { content: String, font_size: f64 },
    Styled { style: Style, child: Arc<DiagramNode> },
    /// Apply an affine transform to the child (lazy; applied at render time).
    Transformed { affine: Affine, child: Arc<DiagramNode> },
    /// Superimpose children back-to-front (last = top).
    Group(Vec<Arc<DiagramNode>>),
}

// ── Public Diagram type ───────────────────────────────────────────────────────

/// A composable, immutable vector-graphics diagram.
///
/// Diagrams are a Monoid under `+` (superimposition / `atop`):
/// - [`Diagram::empty()`] is the identity.
/// - `d1 + d2` renders `d2` on top of `d1` with aligned origins.
///
/// Use operator overloads for layout:
/// - `d1 | d2` — horizontal juxtaposition (left | right)
/// - `d1 / d2` — vertical juxtaposition (top / bottom)
#[derive(Clone, Debug)]
pub struct Diagram {
    pub(crate) node: Arc<DiagramNode>,
    /// Cached bounding box in local coordinates (origin = diagram's own origin).
    pub(crate) bbox: BoundingBox,
}

impl Diagram {
    pub fn empty() -> Self {
        Diagram { node: Arc::new(DiagramNode::Empty), bbox: BoundingBox::EMPTY }
    }

    pub fn bbox(&self) -> BoundingBox {
        self.bbox
    }

    // ── Style chaining ────────────────────────────────────────────────────

    fn with_style(self, style: Style) -> Self {
        Diagram {
            bbox: self.bbox,
            node: Arc::new(DiagramNode::Styled { style, child: self.node }),
        }
    }

    /// Set the stroke color. Short alias for Haskell's `lc`.
    pub fn lc(self, c: Color) -> Self { self.stroke_color(c) }
    pub fn stroke_color(self, c: Color) -> Self {
        self.with_style(Style { stroke_color: Some(c), ..Default::default() })
    }

    /// Set the fill color. Short alias for Haskell's `fc`.
    pub fn fc(self, c: Color) -> Self { self.fill_color(c) }
    pub fn fill_color(self, c: Color) -> Self {
        self.with_style(Style { fill_color: Some(c), ..Default::default() })
    }

    /// Set the stroke width. Short alias for Haskell's `lw` / `lwL`.
    pub fn lw(self, w: f64) -> Self { self.stroke_width(w) }
    pub fn stroke_width(self, w: f64) -> Self {
        self.with_style(Style { stroke_width: Some(w), ..Default::default() })
    }

    /// Set the overall opacity (0.0–1.0).
    pub fn opacity(self, o: f64) -> Self {
        self.with_style(Style { opacity: Some(o), ..Default::default() })
    }

    /// Set a stroke dash pattern. `dashes` are on/off lengths; `offset` is the
    /// phase offset into the pattern. Corresponds to Haskell's `dashingL`.
    pub fn dashing(self, dashes: Vec<f64>, offset: f64) -> Self {
        self.with_style(Style {
            dash: Some(crate::style::DashPattern { dashes, offset }),
            ..Default::default()
        })
    }

    /// Set bold text rendering (affects Text nodes inside this diagram).
    pub fn bold(self) -> Self {
        self.with_style(Style { bold: Some(true), ..Default::default() })
    }

    /// Place a solid background rectangle (filled with `color`) behind this diagram.
    /// Corresponds to Haskell's `# bg color`.
    pub fn bg(self, color: Color) -> Self {
        if let Some(r) = self.bbox.rect() {
            let bg = Diagram::from_node(
                DiagramNode::Rect { width: r.width(), height: r.height() },
                self.bbox,
            )
            .fc(color)
            .stroke_width(0.0)
            .translate(r.center().x, r.center().y);
            bg + self
        } else {
            self
        }
    }

    // ── Transforms ────────────────────────────────────────────────────────

    fn affine(self, aff: Affine) -> Self {
        Diagram {
            bbox: self.bbox.transform(aff),
            node: Arc::new(DiagramNode::Transformed { affine: aff, child: self.node }),
        }
    }

    pub fn translate(self, dx: f64, dy: f64) -> Self {
        let v = Vec2::new(dx, dy);
        Diagram {
            bbox: self.bbox.translate(v),
            node: Arc::new(DiagramNode::Transformed {
                affine: Affine::translate(v),
                child: self.node,
            }),
        }
    }

    pub fn translate_x(self, dx: f64) -> Self { self.translate(dx, 0.0) }
    pub fn translate_y(self, dy: f64) -> Self { self.translate(0.0, dy) }

    pub fn scale(self, s: f64) -> Self {
        self.affine(Affine::scale(s))
    }

    pub fn scale_xy(self, sx: f64, sy: f64) -> Self {
        self.affine(Affine::scale_non_uniform(sx, sy))
    }

    pub fn rotate(self, angle_radians: f64) -> Self {
        self.affine(Affine::rotate(angle_radians))
    }

    /// Rotate by `turns` full rotations (0.25 = 90°, 0.5 = 180°, etc.).
    pub fn rotate_by(self, turns: f64) -> Self {
        self.rotate(turns * std::f64::consts::TAU)
    }

    pub fn reflect_x(self) -> Self { self.scale_xy(1.0, -1.0) }
    pub fn reflect_y(self) -> Self { self.scale_xy(-1.0, 1.0) }

    // ── Alignment ─────────────────────────────────────────────────────────

    /// Shift so the left edge of the bounding box is at x = 0.
    pub fn align_left(self) -> Self {
        if let Some(r) = self.bbox.rect() {
            self.translate_x(-r.x0)
        } else {
            self
        }
    }

    /// Shift so the right edge of the bounding box is at x = 0.
    pub fn align_right(self) -> Self {
        if let Some(r) = self.bbox.rect() {
            self.translate_x(-r.x1)
        } else {
            self
        }
    }

    /// Shift so the top edge of the bounding box is at y = 0.
    pub fn align_top(self) -> Self {
        if let Some(r) = self.bbox.rect() {
            self.translate_y(-r.y0)
        } else {
            self
        }
    }

    /// Shift so the bottom edge of the bounding box is at y = 0.
    pub fn align_bottom(self) -> Self {
        if let Some(r) = self.bbox.rect() {
            self.translate_y(-r.y1)
        } else {
            self
        }
    }

    /// Center horizontally (shift so bbox center x = 0).
    pub fn center_x(self) -> Self {
        if let Some(r) = self.bbox.rect() {
            self.translate_x(-r.center().x)
        } else {
            self
        }
    }

    /// Center vertically (shift so bbox center y = 0).
    pub fn center_y(self) -> Self {
        if let Some(r) = self.bbox.rect() {
            self.translate_y(-r.center().y)
        } else {
            self
        }
    }

    // ── Internal constructor ──────────────────────────────────────────────

    pub(crate) fn from_node(node: DiagramNode, bbox: BoundingBox) -> Self {
        Diagram { node: Arc::new(node), bbox }
    }
}

impl Default for Diagram {
    fn default() -> Self { Diagram::empty() }
}

/// Superimpose `rhs` on top of `lhs` (origins coincide). Haskell's `atop`/`(<>)`.
impl std::ops::Add for Diagram {
    type Output = Diagram;

    fn add(self, rhs: Diagram) -> Diagram {
        let bbox = self.bbox.union(rhs.bbox);
        let mut children: Vec<Arc<DiagramNode>> = Vec::new();
        for node in [self.node, rhs.node] {
            match node.as_ref() {
                DiagramNode::Empty => {}
                DiagramNode::Group(ch) => children.extend(ch.iter().cloned()),
                _ => children.push(node),
            }
        }
        Diagram { node: Arc::new(DiagramNode::Group(children)), bbox }
    }
}

impl std::ops::AddAssign for Diagram {
    fn add_assign(&mut self, rhs: Diagram) {
        *self = self.clone() + rhs;
    }
}

// ── Public constructors ───────────────────────────────────────────────────────

/// Convert a [`Trail`] into a stroked (open) [`Diagram`]. Haskell's `strokeT`.
pub fn stroke_trail(trail: Trail) -> Diagram {
    let pts = trail.to_points(Point::ORIGIN);
    let bbox = BoundingBox::from_points(&pts);
    Diagram::from_node(DiagramNode::StrokedTrail { trail, start: Point::ORIGIN }, bbox)
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trail::hrule;

    #[test]
    fn empty_has_empty_bbox() {
        assert_eq!(Diagram::empty().bbox, BoundingBox::EMPTY);
    }

    #[test]
    fn atop_unions_bboxes() {
        use crate::primitives::{circle, rect};
        let c = circle(1.0);
        let r = rect(4.0, 2.0);
        let combined = c + r;
        let bbox = combined.bbox.rect().unwrap();
        assert!(bbox.width() >= 4.0 - 1e-9);
        assert!(bbox.height() >= 2.0 - 1e-9);
    }

    #[test]
    fn translate_shifts_bbox() {
        use approx::assert_relative_eq;
        let d = stroke_trail(hrule(2.0)).translate(3.0, 4.0);
        let r = d.bbox.rect().unwrap();
        assert_relative_eq!(r.x0, 3.0);
        assert_relative_eq!(r.y0, 4.0);
        assert_relative_eq!(r.x1, 5.0);
    }

    #[test]
    fn style_chaining_preserves_bbox() {
        let d = stroke_trail(hrule(1.0));
        let bbox_before = d.bbox;
        let d = d.lc(crate::style::RED).opacity(0.5);
        assert_eq!(d.bbox, bbox_before);
    }
}
