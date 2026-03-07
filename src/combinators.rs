//! Layout combinators for composing diagrams spatially.
//!
//! These mirror Haskell diagrams' `beside`, `hcat`, `vcat`, and `hcat'`/`vcat'`
//! with a separator.

use kurbo::Vec2;

use crate::diagram::Diagram;

// ── Directions ────────────────────────────────────────────────────────────────

pub const RIGHT: Vec2 = Vec2::new(1.0, 0.0);
pub const LEFT: Vec2 = Vec2::new(-1.0, 0.0);
pub const DOWN: Vec2 = Vec2::new(0.0, 1.0);
pub const UP: Vec2 = Vec2::new(0.0, -1.0);

// ── beside ────────────────────────────────────────────────────────────────────

/// Place `d2` beside `d1` in the given direction, touching envelopes.
///
/// The result's local origin is `d1`'s origin (unchanged). Corresponds to
/// Haskell's `beside`.
pub fn beside(dir: Vec2, d1: Diagram, d2: Diagram) -> Diagram {
    let len = dir.length();
    if len == 0.0 {
        return d1 + d2;
    }
    let unit = dir / len;

    let reach1 = d1.bbox().extent_in(unit).unwrap_or(0.0);
    let reach2 = d2.bbox().extent_in(-unit).unwrap_or(0.0);
    let offset = unit * (reach1 + reach2);

    let d2 = d2.translate(offset.x, offset.y);
    d1 + d2
}

/// Superimpose two diagrams (origins coincide; `d2` on top). Same as `d1 + d2`.
pub fn atop(d1: Diagram, d2: Diagram) -> Diagram {
    d1 + d2
}

// ── hcat / vcat ───────────────────────────────────────────────────────────────

/// Place diagrams side by side horizontally (left to right), touching.
///
/// Corresponds to Haskell's `hcat`.
pub fn hcat(diagrams: impl IntoIterator<Item = Diagram>) -> Diagram {
    diagrams
        .into_iter()
        .fold(Diagram::empty(), |acc, d| beside(RIGHT, acc, d))
}

/// Place diagrams top to bottom vertically, touching.
///
/// Corresponds to Haskell's `vcat`.
pub fn vcat(diagrams: impl IntoIterator<Item = Diagram>) -> Diagram {
    diagrams
        .into_iter()
        .fold(Diagram::empty(), |acc, d| beside(DOWN, acc, d))
}

/// Horizontal concatenation with a `sep`-unit gap between each pair.
///
/// Corresponds to Haskell's `hcat' (with & sep .~ sep)`.
pub fn hcat_sep(sep: f64, diagrams: impl IntoIterator<Item = Diagram>) -> Diagram {
    let mut iter = diagrams.into_iter().peekable();
    let mut acc = match iter.next() {
        None => return Diagram::empty(),
        Some(d) => d,
    };
    for d in iter {
        acc = beside(RIGHT, acc, crate::primitives::strut_x(sep));
        acc = beside(RIGHT, acc, d);
    }
    acc
}

/// Vertical concatenation with a `sep`-unit gap between each pair.
///
/// Corresponds to Haskell's `vcat' (with & sep .~ sep)`.
pub fn vcat_sep(sep: f64, diagrams: impl IntoIterator<Item = Diagram>) -> Diagram {
    let mut iter = diagrams.into_iter().peekable();
    let mut acc = match iter.next() {
        None => return Diagram::empty(),
        Some(d) => d,
    };
    for d in iter {
        acc = beside(DOWN, acc, crate::primitives::strut_y(sep));
        acc = beside(DOWN, acc, d);
    }
    acc
}

/// Place diagrams at explicit absolute positions (each at the given `Point`).
///
/// Corresponds to Haskell's `position`.
pub fn position(placed: impl IntoIterator<Item = (kurbo::Point, Diagram)>) -> Diagram {
    placed
        .into_iter()
        .fold(Diagram::empty(), |acc, (pt, d)| acc + d.translate(pt.x, pt.y))
}

// ── Operator overloads ────────────────────────────────────────────────────────

/// `d1 | d2` — horizontal juxtaposition (Haskell's `|||`).
impl std::ops::BitOr for Diagram {
    type Output = Diagram;
    fn bitor(self, rhs: Diagram) -> Diagram {
        beside(RIGHT, self, rhs)
    }
}

/// `d1 / d2` — vertical juxtaposition (Haskell's `===`).
impl std::ops::Div for Diagram {
    type Output = Diagram;
    fn div(self, rhs: Diagram) -> Diagram {
        beside(DOWN, self, rhs)
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::{circle, rect, strut_x};
    use approx::assert_relative_eq;

    #[test]
    fn beside_two_unit_circles_horizontally() {
        // Two unit circles placed side by side should span 4 units wide.
        let d = circle(1.0) | circle(1.0);
        let r = d.bbox.rect().unwrap();
        assert_relative_eq!(r.width(), 4.0, epsilon = 1e-10);
    }

    #[test]
    fn beside_two_unit_circles_vertically() {
        let d = circle(1.0) / circle(1.0);
        let r = d.bbox.rect().unwrap();
        assert_relative_eq!(r.height(), 4.0, epsilon = 1e-10);
    }

    #[test]
    fn hcat_three_circles() {
        let d = hcat(vec![circle(1.0), circle(1.0), circle(1.0)]);
        let r = d.bbox.rect().unwrap();
        assert_relative_eq!(r.width(), 6.0, epsilon = 1e-10);
    }

    #[test]
    fn hcat_sep_adds_gaps() {
        // Two unit circles with sep=2 should span: 2 + 2 + 2 = 6...
        // wait: circle has r=1 so diameter=2, sep=2 between them → total = 2+2+2=6
        let d = hcat_sep(2.0, vec![circle(1.0), circle(1.0)]);
        let r = d.bbox.rect().unwrap();
        assert_relative_eq!(r.width(), 6.0, epsilon = 1e-10);
    }

    #[test]
    fn vcat_sep_adds_gaps() {
        let d = vcat_sep(1.0, vec![rect(2.0, 1.0), rect(2.0, 1.0)]);
        let r = d.bbox.rect().unwrap();
        assert_relative_eq!(r.height(), 3.0, epsilon = 1e-10);
    }

    #[test]
    fn beside_empty_diagram() {
        let d = circle(1.0) | Diagram::empty();
        let r = d.bbox.rect().unwrap();
        assert_relative_eq!(r.width(), 2.0, epsilon = 1e-10);
    }
}
