//! The [`Diagram`] type: the central composable drawing primitive.
//!
//! A `Diagram` is an immutable scene-tree node. It is a Monoid:
//! - [`Diagram::empty()`] / `Default::default()` is the identity.
//! - `d1 + d2` superimposes `d2` on top of `d1` (origins coincide).

use std::sync::Arc;

use kurbo::Point;

use crate::style::{Color, Style};
use crate::trail::Trail;

// ── Internal scene tree ───────────────────────────────────────────────────────

/// The internal recursive scene-tree node.
#[derive(Clone, Debug)]
pub(crate) enum DiagramNode {
    /// An empty diagram (identity element).
    Empty,
    /// A stroked (open) trail starting at `start`.
    StrokedTrail { trail: Trail, start: Point },
    /// Override style on a child diagram.
    Styled { style: Style, child: Arc<DiagramNode> },
    /// Superimpose a list of diagrams back-to-front (last = top).
    Group(Vec<Arc<DiagramNode>>),
}

// ── Public Diagram type ───────────────────────────────────────────────────────

/// A composable vector-graphics diagram.
///
/// Diagrams form a Monoid under `+` (superimposition, `atop`):
/// ```
/// use hagoromo::{Diagram, stroke_trail, hrule};
/// let empty: Diagram = Diagram::empty();
/// let d = stroke_trail(hrule(1.0));
/// let combined = empty + d;
/// ```
#[derive(Clone, Debug)]
pub struct Diagram {
    pub(crate) node: Arc<DiagramNode>,
}

impl Diagram {
    /// The empty diagram (no visual content).
    pub fn empty() -> Self {
        Diagram { node: Arc::new(DiagramNode::Empty) }
    }

    fn styled(self, style: Style) -> Self {
        Diagram {
            node: Arc::new(DiagramNode::Styled { style, child: self.node }),
        }
    }

    // ── Style chaining ────────────────────────────────────────────────────

    /// Set the line (stroke) color. Short alias matching Haskell's `lc`.
    pub fn lc(self, color: Color) -> Self {
        self.stroke_color(color)
    }

    /// Set the line (stroke) color.
    pub fn stroke_color(self, color: Color) -> Self {
        self.styled(Style { stroke_color: Some(color), ..Default::default() })
    }

    /// Set the fill color. Short alias matching Haskell's `fc`.
    pub fn fc(self, color: Color) -> Self {
        self.fill_color(color)
    }

    /// Set the fill color.
    pub fn fill_color(self, color: Color) -> Self {
        self.styled(Style { fill_color: Some(color), ..Default::default() })
    }

    /// Set the stroke width. Short alias matching Haskell's `lw`.
    pub fn lw(self, width: f64) -> Self {
        self.stroke_width(width)
    }

    /// Set the stroke width.
    pub fn stroke_width(self, width: f64) -> Self {
        self.styled(Style { stroke_width: Some(width), ..Default::default() })
    }

    /// Set the overall opacity (0.0 = transparent, 1.0 = opaque).
    pub fn opacity(self, o: f64) -> Self {
        self.styled(Style { opacity: Some(o), ..Default::default() })
    }
}

impl Default for Diagram {
    fn default() -> Self {
        Diagram::empty()
    }
}

/// Superimpose `rhs` on top of `lhs` (origins coincide).
/// Corresponds to Haskell's `atop` / `(<>)` on diagrams.
impl std::ops::Add for Diagram {
    type Output = Diagram;

    fn add(self, rhs: Diagram) -> Diagram {
        // Flatten consecutive groups to avoid deep nesting.
        let mut children: Vec<Arc<DiagramNode>> = Vec::new();
        for node in [self.node, rhs.node] {
            match node.as_ref() {
                DiagramNode::Empty => {}
                DiagramNode::Group(ch) => children.extend(ch.iter().cloned()),
                _ => children.push(node),
            }
        }
        Diagram { node: Arc::new(DiagramNode::Group(children)) }
    }
}

// ── stroke_trail ──────────────────────────────────────────────────────────────

/// Convert a [`Trail`] into a stroked [`Diagram`] (open path, no fill).
///
/// The trail is placed with its start at the origin. Use style-chaining
/// methods (`.lc()`, `.lw()`, `.opacity()`) to configure its appearance.
///
/// Corresponds to Haskell's `strokeT`.
pub fn stroke_trail(trail: Trail) -> Diagram {
    Diagram {
        node: Arc::new(DiagramNode::StrokedTrail { trail, start: Point::ORIGIN }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trail::hrule;

    #[test]
    fn empty_plus_diagram_is_diagram() {
        let d = stroke_trail(hrule(1.0));
        let combined = Diagram::empty() + d;
        // Should not be an Empty node
        assert!(!matches!(combined.node.as_ref(), DiagramNode::Empty));
    }

    #[test]
    fn style_chaining_wraps_node() {
        let d = stroke_trail(hrule(1.0)).lc(crate::style::RED).opacity(0.5);
        // Should be doubly wrapped in Styled nodes
        assert!(matches!(d.node.as_ref(), DiagramNode::Styled { .. }));
    }
}
