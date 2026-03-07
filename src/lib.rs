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
pub mod combinators;
pub mod backend;
pub mod backends;

// Core types
pub use trail::{Trail, hrule, vrule};
pub use style::{Color, DashPattern, Style, BLACK, BLUE, GREEN, RED, SILVER, WHITE};
pub use diagram::{Diagram, stroke_trail};
pub use envelope::BoundingBox;

// Primitives
pub use primitives::{circle, rect, square, strut_x, strut_y, text};

// Layout combinators
pub use combinators::{atop, beside, hcat, hcat_sep, position, vcat, vcat_sep};
pub use combinators::{DOWN, LEFT, RIGHT, UP};

// Backend / rendering
pub use backend::{render_svg, RenderOptions};
