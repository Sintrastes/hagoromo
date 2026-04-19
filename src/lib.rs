//! Hagoromo — a declarative vector graphics library for Rust,
//! inspired by the Haskell [`diagrams`](https://hackage.haskell.org/package/diagrams) package.
//!
//! # Quick start
//!
//! ```rust
//! use hagoromo::*;
//!
//! fn hilbert(n: u32) -> Trail {
//!     if n == 0 { return Trail::default(); }
//!     hilbert_prime(n - 1).reflect_y()
//!         + vrule(1.0) + hilbert(n - 1) + hrule(1.0)
//!         + hilbert(n - 1) + vrule(-1.0)
//!         + hilbert_prime(n - 1).reflect_x()
//! }
//! fn hilbert_prime(n: u32) -> Trail { hilbert(n).rotate_by(0.25) }
//!
//! let svg = render_svg(
//!     &stroke_trail(hilbert(3)).lc(SILVER).opacity(0.3),
//!     &RenderOptions::default(),
//! );
//! ```

pub mod trail;
pub mod style;
pub mod diagram;
pub mod envelope;
pub mod primitives;
pub mod spline;
pub mod combinators;
pub mod backend;
pub mod backends;

// Core types
pub use trail::{Trail, hrule, vrule};
pub use style::{Color, DashPattern, GradientStop, Measure, RadialGradient, Style};
pub use style::{BLACK, BLUE, GREEN, RED, SILVER, WHITE};
pub use style::{NONE, ULTRA_THIN, VERY_THIN, THIN, MEDIUM, THICK, VERY_THICK, ULTRA_THICK};
pub use diagram::{Diagram, stroke_spline, stroke_trail};
pub use envelope::BoundingBox;

// Primitives
pub use primitives::{circle, equilateral_triangle, polygon, polyline, rect, reg_poly, square, strut_x, strut_y, text};
pub use kurbo::{Point, Vec2};

// Spline
pub use spline::{CubicSpline, cubic_spline};

// Layout combinators
pub use combinators::{appends, atop, beside, hcat, hcat_sep, position, vcat, vcat_sep};
pub use combinators::{DOWN, LEFT, RIGHT, UP};

// Backend / rendering
pub use backend::{render_svg, RenderOptions};

/// Vertically concatenate diagrams without needing an explicit `vec![]`.
///
/// ```rust
/// use hagoromo::*;
/// let d = vcat![circle(1.0), rect(2.0, 1.0), square(1.0)];
/// ```
#[macro_export]
macro_rules! vcat {
    ($($d:expr),* $(,)?) => { $crate::vcat(vec![$($d),*]) };
}

/// Horizontally concatenate diagrams without needing an explicit `vec![]`.
///
/// ```rust
/// use hagoromo::*;
/// let d = hcat![circle(1.0), rect(2.0, 1.0), square(1.0)];
/// ```
#[macro_export]
macro_rules! hcat {
    ($($d:expr),* $(,)?) => { $crate::hcat(vec![$($d),*]) };
}
