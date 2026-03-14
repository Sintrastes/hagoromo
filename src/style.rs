//! Style types: colors and stroke/fill attributes.

/// An RGBA color with components in [0.0, 1.0].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    /// Construct from float components in [0.0, 1.0].
    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color { r, g, b, a }
    }

    /// Construct an opaque color from float components in [0.0, 1.0].
    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Color { r, g, b, a: 1.0 }
    }

    /// Convert to an SVG color string: either `"none"` (fully transparent) or
    /// a hex string like `"#c0c0c0"`.
    pub fn to_svg_color(&self) -> String {
        if self.a == 0.0 {
            "none".to_string()
        } else {
            format!(
                "#{:02x}{:02x}{:02x}",
                (self.r * 255.0).round() as u8,
                (self.g * 255.0).round() as u8,
                (self.b * 255.0).round() as u8,
            )
        }
    }

    /// Parse an opaque color from a CSS hex string like `"#5E0042"` or `"5E0042"`.
    pub fn from_hex(hex: &str) -> Self {
        let hex = hex.trim_start_matches('#');
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
        Color::rgb(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
    }

    /// The alpha component as an SVG opacity value (0.0–1.0).
    pub fn alpha(&self) -> f32 {
        self.a
    }
}

// ── Stroke-width measure ─────────────────────────────────────────────────────

/// A stroke-width value that can be either absolute (diagram units) or
/// normalized (a fraction of the rendered diagram's larger dimension).
///
/// Matches Haskell diagrams' `Measure` / `normalized` / `output` system.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Measure {
    /// Absolute width in diagram coordinate units.
    Absolute(f64),
    /// Width as a fraction of `max(diagram_width, diagram_height)`.
    Normalized(f64),
}

impl Measure {
    /// Resolve to an absolute width given the diagram's larger dimension.
    pub fn resolve(self, diagram_size: f64) -> f64 {
        match self {
            Measure::Absolute(w) => w,
            Measure::Normalized(f) => f * diagram_size,
        }
    }
}

// Named stroke-width constants (mirror Haskell diagrams' predefined widths).
pub const NONE: Measure      = Measure::Absolute(0.0);
pub const ULTRA_THIN: Measure = Measure::Normalized(0.0005);
pub const VERY_THIN: Measure  = Measure::Normalized(0.001);
pub const THIN: Measure       = Measure::Normalized(0.002);
pub const MEDIUM: Measure     = Measure::Normalized(0.005);
pub const THICK: Measure      = Measure::Normalized(0.01);
pub const VERY_THICK: Measure = Measure::Normalized(0.02);
pub const ULTRA_THICK: Measure = Measure::Normalized(0.04);

// ── Named color constants ────────────────────────────────────────────────────

pub const BLACK: Color = Color::rgb(0.0, 0.0, 0.0);
pub const WHITE: Color = Color::rgb(1.0, 1.0, 1.0);
pub const RED: Color = Color::rgb(1.0, 0.0, 0.0);
pub const GREEN: Color = Color::rgb(0.0, 0.502, 0.0);
pub const BLUE: Color = Color::rgb(0.0, 0.0, 1.0);
/// CSS silver: #C0C0C0
pub const SILVER: Color = Color::rgb(0.753, 0.753, 0.753);
pub const TRANSPARENT: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);

// ── Style ────────────────────────────────────────────────────────────────────

// ── Radial gradient ───────────────────────────────────────────────────────────

/// A single color stop in a gradient.
#[derive(Clone, Debug)]
pub struct GradientStop {
    /// Position along the gradient axis, in [0.0, 1.0].
    pub offset: f64,
    pub color: Color,
    pub opacity: f64,
}

/// A radial gradient from a center point outward to a given radius.
///
/// Coordinates are in diagram units (`gradientUnits="userSpaceOnUse"`).
#[derive(Clone, Debug)]
pub struct RadialGradient {
    /// Center of the gradient in diagram coordinates.
    pub cx: f64,
    pub cy: f64,
    /// Outer radius in diagram coordinates.
    pub r: f64,
    pub stops: Vec<GradientStop>,
}

impl RadialGradient {
    /// Create a radial gradient centered at the origin with given radius and stops.
    pub fn new(r: f64, stops: Vec<GradientStop>) -> Self {
        RadialGradient { cx: 0.0, cy: 0.0, r, stops }
    }
}

// ── Stroke dash pattern ───────────────────────────────────────────────────────

/// A stroke dash pattern.
#[derive(Clone, Debug)]
pub struct DashPattern {
    /// On/off dash lengths in diagram units.
    pub dashes: Vec<f64>,
    /// Phase offset into the pattern.
    pub offset: f64,
}

/// Rendering style for a diagram element.
///
/// All fields are `Option` so that partial styles can be accumulated and
/// resolved with an outer default.
#[derive(Clone, Debug, Default)]
pub struct Style {
    pub fill_color: Option<Color>,
    pub stroke_color: Option<Color>,
    pub stroke_width: Option<f64>,
    pub opacity: Option<f64>,
    pub dash: Option<DashPattern>,
    pub bold: Option<bool>,
    /// Radial gradient fill; overrides `fill_color` when present.
    pub fill_gradient: Option<RadialGradient>,
    /// CSS `font-family` value for text nodes (e.g. `"Bravura, serif"`).
    pub font_family: Option<String>,
}

impl Style {
    /// Merge `self` (outer/default) with `inner` (more specific).
    /// Fields present in `inner` take precedence.
    pub fn merge_over(&self, inner: &Style) -> Style {
        Style {
            fill_color: inner.fill_color.or(self.fill_color),
            stroke_color: inner.stroke_color.or(self.stroke_color),
            stroke_width: inner.stroke_width.or(self.stroke_width),
            opacity: inner.opacity.or(self.opacity),
            dash: inner.dash.clone().or_else(|| self.dash.clone()),
            bold: inner.bold.or(self.bold),
            fill_gradient: inner.fill_gradient.clone().or_else(|| self.fill_gradient.clone()),
            font_family: inner.font_family.clone().or_else(|| self.font_family.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_svg_color_formats_hex() {
        assert_eq!(BLACK.to_svg_color(), "#000000");
        assert_eq!(WHITE.to_svg_color(), "#ffffff");
        assert_eq!(SILVER.to_svg_color(), "#c0c0c0");
    }

    #[test]
    fn merge_inner_wins() {
        let outer = Style { stroke_color: Some(BLACK), stroke_width: Some(1.0), ..Default::default() };
        let inner = Style { stroke_color: Some(RED), ..Default::default() };
        let merged = outer.merge_over(&inner);
        assert_eq!(merged.stroke_color, Some(RED));
        assert_eq!(merged.stroke_width, Some(1.0)); // falls through from outer
    }
}
