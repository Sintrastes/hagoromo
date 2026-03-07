//! Hagoromo — a declarative vector graphics library for Rust,
//! inspired by the Haskell [`diagrams`](https://hackage.haskell.org/package/diagrams) package.
//!
//! # Quick start
//!
//! ```rust
//! use hagoromo::{Trail, hrule, vrule, stroke_trail, SILVER, RenderOptions, render_svg};
//!
//! fn hilbert(n: u32) -> Trail {
//!     if n == 0 { return Trail::default(); }
//!     hilbert_prime(n - 1).reflect_y()
//!         + vrule(1.0)
//!         + hilbert(n - 1)
//!         + hrule(1.0)
//!         + hilbert(n - 1)
//!         + vrule(-1.0)
//!         + hilbert_prime(n - 1).reflect_x()
//! }
//!
//! fn hilbert_prime(n: u32) -> Trail {
//!     hilbert(n).rotate_by(0.25)
//! }
//!
//! let svg = render_svg(
//!     &stroke_trail(hilbert(3)).lc(SILVER).opacity(0.3),
//!     &RenderOptions::default(),
//! );
//! ```

pub mod trail;
pub mod style;
pub mod diagram;
pub mod backend;
pub mod backends;

pub use trail::{Trail, hrule, vrule};
pub use style::{Color, Style, BLACK, WHITE, RED, GREEN, BLUE, SILVER};
pub use diagram::{Diagram, stroke_trail};
pub use backend::{RenderOptions, render_svg};
