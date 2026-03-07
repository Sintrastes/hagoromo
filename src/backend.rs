//! The [`Backend`] trait and rendering helpers.

use crate::backends::svg::SvgBackend;
use crate::diagram::Diagram;
use crate::style::Color;

/// Options that control how a diagram is rendered.
#[derive(Clone, Debug)]
pub struct RenderOptions {
    /// Padding (in diagram units) added around the bounding box.
    pub padding: f64,
    /// Optional background color. `None` means transparent.
    pub background: Option<Color>,
    /// Default stroke width used when none is set on the diagram.
    pub default_stroke_width: f64,
}

impl Default for RenderOptions {
    fn default() -> Self {
        RenderOptions {
            padding: 2.0,
            background: None,
            default_stroke_width: 0.5,
        }
    }
}

/// The rendering interface. Backends implement this to produce different output
/// formats from the same diagram.
pub trait Backend {
    /// Output type (e.g. `String` for SVG).
    type Output;
    /// Error type.
    type Error: std::error::Error;

    fn render(&mut self, diagram: &Diagram, opts: &RenderOptions) -> Result<Self::Output, Self::Error>;
}

/// Render a diagram to an SVG string using the default SVG backend.
pub fn render_svg(diagram: &Diagram, opts: &RenderOptions) -> String {
    SvgBackend::new().render(diagram, opts).unwrap()
}
