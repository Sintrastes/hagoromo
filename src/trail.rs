//! The [`Trail`] type: a sequence of linear segments forming a path.
//!
//! Trails are the primary building block for open paths (like curves drawn
//! without a fill). They are a Monoid: [`Trail::default()`] is the empty
//! trail and `+` concatenates trails end-to-end.
//!
//! This mirrors Haskell diagrams' `Trail` type.

use kurbo::{Point, Vec2};

/// A trail is an ordered sequence of linear segments, each specified as a
/// relative displacement from the previous endpoint.
///
/// Trails form a Monoid:
/// - [`Trail::default()`] / [`Trail::empty()`] is the identity (no segments).
/// - `t1 + t2` concatenates `t1` and `t2` end-to-end.
///
/// All transformations (`reflect_x`, `reflect_y`, `rotate_by`) consume `self`
/// and return a new `Trail`, enabling method chaining.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Trail {
    pub(crate) segments: Vec<Vec2>,
}

impl Trail {
    /// The empty trail (no segments).
    pub fn empty() -> Self {
        Trail { segments: vec![] }
    }

    /// Apply a function to every segment vector.
    fn map_segments(self, f: impl Fn(Vec2) -> Vec2) -> Self {
        Trail {
            segments: self.segments.into_iter().map(f).collect(),
        }
    }

    /// Reflect across the X-axis: `(x, y) → (x, −y)`.
    ///
    /// Corresponds to Haskell's `reflectX`.
    pub fn reflect_x(self) -> Self {
        self.map_segments(|v| Vec2::new(v.x, -v.y))
    }

    /// Reflect across the Y-axis: `(x, y) → (−x, y)`.
    ///
    /// Corresponds to Haskell's `reflectY`.
    pub fn reflect_y(self) -> Self {
        self.map_segments(|v| Vec2::new(-v.x, v.y))
    }

    /// Rotate by `turns` full rotations (positive = counter-clockwise in
    /// mathematical coordinates; clockwise in SVG screen coordinates).
    ///
    /// `rotate_by(0.25)` = 90°, `rotate_by(0.5)` = 180°, etc.
    ///
    /// Corresponds to Haskell's `rotateBy`.
    pub fn rotate_by(self, turns: f64) -> Self {
        let angle = turns * std::f64::consts::TAU;
        let (sin_a, cos_a) = angle.sin_cos();
        self.map_segments(|v| Vec2::new(cos_a * v.x - sin_a * v.y, sin_a * v.x + cos_a * v.y))
    }

    /// Compute the total displacement of the trail (sum of all segments).
    pub fn total_displacement(&self) -> Vec2 {
        self.segments.iter().copied().fold(Vec2::ZERO, |acc, v| acc + v)
    }

    /// Return all absolute points along the trail starting from `start`.
    /// The returned vec has `segments.len() + 1` elements.
    pub fn to_points(&self, start: Point) -> Vec<Point> {
        let mut pts = Vec::with_capacity(self.segments.len() + 1);
        let mut cur = start;
        pts.push(cur);
        for &seg in &self.segments {
            cur += seg;
            pts.push(cur);
        }
        pts
    }

    /// Number of segments in the trail.
    pub fn len(&self) -> usize {
        self.segments.len()
    }

    /// Whether the trail has no segments.
    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }
}

/// Concatenate two trails end-to-end. Corresponds to Haskell's `(<>)` on trails.
impl std::ops::Add for Trail {
    type Output = Trail;

    fn add(mut self, other: Trail) -> Trail {
        self.segments.extend(other.segments);
        self
    }
}

impl std::ops::AddAssign for Trail {
    fn add_assign(&mut self, other: Trail) {
        self.segments.extend(other.segments);
    }
}

/// A horizontal line segment of the given `length`.
///
/// Corresponds to Haskell's `hrule`.
pub fn hrule(length: f64) -> Trail {
    Trail { segments: vec![Vec2::new(length, 0.0)] }
}

/// A vertical line segment of the given `length` (positive = downward in SVG).
///
/// Corresponds to Haskell's `vrule`.
pub fn vrule(length: f64) -> Trail {
    Trail { segments: vec![Vec2::new(0.0, length)] }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn empty_trail_has_no_segments() {
        assert_eq!(Trail::empty().len(), 0);
    }

    #[test]
    fn hrule_produces_horizontal_segment() {
        let t = hrule(3.0);
        assert_eq!(t.len(), 1);
        assert_relative_eq!(t.segments[0].x, 3.0);
        assert_relative_eq!(t.segments[0].y, 0.0);
    }

    #[test]
    fn vrule_produces_vertical_segment() {
        let t = vrule(2.0);
        assert_eq!(t.len(), 1);
        assert_relative_eq!(t.segments[0].x, 0.0);
        assert_relative_eq!(t.segments[0].y, 2.0);
    }

    #[test]
    fn concatenation_appends_segments() {
        let t = hrule(1.0) + vrule(1.0) + hrule(-1.0);
        assert_eq!(t.len(), 3);
    }

    #[test]
    fn empty_is_identity_for_add() {
        let t = hrule(1.0);
        let left = Trail::empty() + t.clone();
        let right = t.clone() + Trail::empty();
        assert_eq!(left.len(), 1);
        assert_eq!(right.len(), 1);
    }

    #[test]
    fn reflect_x_negates_y() {
        let t = vrule(1.0).reflect_x();
        assert_relative_eq!(t.segments[0].x, 0.0);
        assert_relative_eq!(t.segments[0].y, -1.0);
    }

    #[test]
    fn reflect_y_negates_x() {
        let t = hrule(1.0).reflect_y();
        assert_relative_eq!(t.segments[0].x, -1.0);
        assert_relative_eq!(t.segments[0].y, 0.0);
    }

    #[test]
    fn rotate_by_quarter_turn_rotates_right_to_down() {
        // Rotating a rightward segment by 0.25 turns (90° CW in screen coords)
        // should produce a downward segment.
        let t = hrule(1.0).rotate_by(0.25);
        assert_relative_eq!(t.segments[0].x, 0.0, epsilon = 1e-10);
        assert_relative_eq!(t.segments[0].y, 1.0, epsilon = 1e-10);
    }

    #[test]
    fn to_points_accumulates_correctly() {
        let t = hrule(1.0) + vrule(1.0);
        let pts = t.to_points(Point::ORIGIN);
        assert_eq!(pts.len(), 3);
        assert_relative_eq!(pts[0].x, 0.0);
        assert_relative_eq!(pts[0].y, 0.0);
        assert_relative_eq!(pts[1].x, 1.0);
        assert_relative_eq!(pts[1].y, 0.0);
        assert_relative_eq!(pts[2].x, 1.0);
        assert_relative_eq!(pts[2].y, 1.0);
    }

    #[test]
    fn hilbert_1_has_three_segments() {
        // This validates the Hilbert curve logic at the Trail level.
        // hilbert(1) = vrule(1) + hrule(1) + vrule(-1)
        // (the hilbert_prime(0) terms are empty, so they contribute nothing)
        let h1 = vrule(1.0) + hrule(1.0) + vrule(-1.0);
        assert_eq!(h1.len(), 3);
    }
}
