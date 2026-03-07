//! Primitive shape constructors.
//!
//! All shapes are centered at the origin by convention (matching Haskell diagrams).

use kurbo::Rect;

use crate::diagram::{Diagram, DiagramNode};
use crate::envelope::BoundingBox;

/// A filled/stroked circle of the given `radius`, centered at the origin.
pub fn circle(radius: f64) -> Diagram {
    Diagram::from_node(
        DiagramNode::Circle { radius },
        BoundingBox::from_rect(Rect::new(-radius, -radius, radius, radius)),
    )
}

/// A rectangle of the given `width` × `height`, centered at the origin.
pub fn rect(width: f64, height: f64) -> Diagram {
    let (hw, hh) = (width / 2.0, height / 2.0);
    Diagram::from_node(
        DiagramNode::Rect { width, height },
        BoundingBox::from_rect(Rect::new(-hw, -hh, hw, hh)),
    )
}

/// A square with the given `side` length, centered at the origin.
pub fn square(side: f64) -> Diagram {
    rect(side, side)
}

/// Text centered at the origin with the given `font_size` (in diagram units).
///
/// The bounding box is approximated as `width ≈ chars * font_size * 0.6,
/// height ≈ font_size`.
pub fn text(content: impl Into<String>, font_size: f64) -> Diagram {
    let s = content.into();
    let approx_w = s.len() as f64 * font_size * 0.6;
    let approx_h = font_size;
    Diagram::from_node(
        DiagramNode::Text { content: s, font_size },
        BoundingBox::from_rect(Rect::new(
            -approx_w / 2.0,
            -approx_h / 2.0,
            approx_w / 2.0,
            approx_h / 2.0,
        )),
    )
}

/// An invisible horizontal spacer of the given `width`. No visual output.
///
/// Useful for adding gaps in `hcat_sep` / `vcat_sep`, or for padding.
/// Corresponds to Haskell's `strutX`.
pub fn strut_x(width: f64) -> Diagram {
    let hw = width / 2.0;
    Diagram::from_node(
        DiagramNode::Empty,
        BoundingBox::from_rect(Rect::new(-hw, 0.0, hw, 0.0)),
    )
}

/// An invisible vertical spacer of the given `height`. No visual output.
///
/// Corresponds to Haskell's `strutY`.
pub fn strut_y(height: f64) -> Diagram {
    let hh = height / 2.0;
    Diagram::from_node(
        DiagramNode::Empty,
        BoundingBox::from_rect(Rect::new(0.0, -hh, 0.0, hh)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn circle_bbox_is_centered() {
        let r = circle(2.0).bbox.rect().unwrap();
        assert_relative_eq!(r.x0, -2.0);
        assert_relative_eq!(r.y0, -2.0);
        assert_relative_eq!(r.x1, 2.0);
        assert_relative_eq!(r.y1, 2.0);
    }

    #[test]
    fn rect_bbox_is_centered() {
        let r = rect(4.0, 2.0).bbox.rect().unwrap();
        assert_relative_eq!(r.x0, -2.0);
        assert_relative_eq!(r.y0, -1.0);
        assert_relative_eq!(r.x1, 2.0);
        assert_relative_eq!(r.y1, 1.0);
    }

    #[test]
    fn strut_x_has_width_but_no_height() {
        let r = strut_x(3.0).bbox.rect().unwrap();
        assert_relative_eq!(r.width(), 3.0);
        assert_relative_eq!(r.height(), 0.0);
    }

    #[test]
    fn strut_y_has_height_but_no_width() {
        let r = strut_y(3.0).bbox.rect().unwrap();
        assert_relative_eq!(r.height(), 3.0);
        assert_relative_eq!(r.width(), 0.0);
    }
}
