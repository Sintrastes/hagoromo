//! [`BoundingBox`]: axis-aligned bounding box used as the diagram envelope.
//!
//! This is a v1 approximation of Haskell diagrams' functional `Envelope`.
//! It is sufficient for layout of axis-aligned compositions; rotated shapes
//! produce a conservative (larger) bounding box.

use kurbo::{Affine, Point, Rect, Vec2};

/// An axis-aligned bounding box in local diagram coordinates.
///
/// `None` represents the empty envelope (no spatial extent). An empty diagram
/// or an invisible spacer has an empty envelope.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BoundingBox(pub(crate) Option<Rect>);

impl BoundingBox {
    pub const EMPTY: Self = BoundingBox(None);

    pub fn from_rect(r: Rect) -> Self {
        BoundingBox(Some(r))
    }

    /// Compute the tight bounding box of a slice of points.
    pub fn from_points(pts: &[Point]) -> Self {
        if pts.is_empty() {
            return Self::EMPTY;
        }
        let mut x0 = pts[0].x;
        let mut y0 = pts[0].y;
        let mut x1 = pts[0].x;
        let mut y1 = pts[0].y;
        for p in pts.iter().skip(1) {
            x0 = x0.min(p.x);
            y0 = y0.min(p.y);
            x1 = x1.max(p.x);
            y1 = y1.max(p.y);
        }
        BoundingBox(Some(Rect::new(x0, y0, x1, y1)))
    }

    pub fn rect(self) -> Option<Rect> {
        self.0
    }

    pub fn union(self, other: Self) -> Self {
        match (self.0, other.0) {
            (None, b) => BoundingBox(b),
            (a, None) => BoundingBox(a),
            (Some(a), Some(b)) => BoundingBox(Some(Rect::new(
                a.x0.min(b.x0),
                a.y0.min(b.y0),
                a.x1.max(b.x1),
                a.y1.max(b.y1),
            ))),
        }
    }

    pub fn translate(self, v: Vec2) -> Self {
        BoundingBox(
            self.0
                .map(|r| Rect::new(r.x0 + v.x, r.y0 + v.y, r.x1 + v.x, r.y1 + v.y)),
        )
    }

    /// Bounding box of the four corners after applying an affine transform.
    pub fn transform(self, aff: Affine) -> Self {
        let r = match self.0 {
            None => return Self::EMPTY,
            Some(r) => r,
        };
        let corners = [
            aff * Point::new(r.x0, r.y0),
            aff * Point::new(r.x1, r.y0),
            aff * Point::new(r.x0, r.y1),
            aff * Point::new(r.x1, r.y1),
        ];
        Self::from_points(&corners)
    }

    /// Maximum extent of the box projected onto `dir` (a unit vector).
    ///
    /// Returns `None` for the empty envelope.
    /// This is the key operation for `beside`-style layout.
    pub fn extent_in(self, dir: Vec2) -> Option<f64> {
        let r = self.0?;
        let corners = [
            Vec2::new(r.x0, r.y0),
            Vec2::new(r.x1, r.y0),
            Vec2::new(r.x0, r.y1),
            Vec2::new(r.x1, r.y1),
        ];
        Some(
            corners
                .iter()
                .map(|c| c.dot(dir))
                .fold(f64::NEG_INFINITY, f64::max),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn union_of_empty_is_empty() {
        assert_eq!(BoundingBox::EMPTY.union(BoundingBox::EMPTY), BoundingBox::EMPTY);
    }

    #[test]
    fn union_with_empty_is_identity() {
        let bb = BoundingBox::from_rect(Rect::new(0.0, 0.0, 1.0, 1.0));
        assert_eq!(bb.union(BoundingBox::EMPTY), bb);
        assert_eq!(BoundingBox::EMPTY.union(bb), bb);
    }

    #[test]
    fn extent_in_rightward() {
        let bb = BoundingBox::from_rect(Rect::new(-1.0, -1.0, 1.0, 1.0));
        assert_relative_eq!(bb.extent_in(Vec2::new(1.0, 0.0)).unwrap(), 1.0);
    }

    #[test]
    fn extent_in_leftward() {
        let bb = BoundingBox::from_rect(Rect::new(-1.0, -1.0, 1.0, 1.0));
        assert_relative_eq!(bb.extent_in(Vec2::new(-1.0, 0.0)).unwrap(), 1.0);
    }

    #[test]
    fn translate_shifts_rect() {
        let bb = BoundingBox::from_rect(Rect::new(0.0, 0.0, 2.0, 2.0));
        let shifted = bb.translate(Vec2::new(3.0, 4.0));
        let r = shifted.rect().unwrap();
        assert_relative_eq!(r.x0, 3.0);
        assert_relative_eq!(r.y0, 4.0);
    }
}
