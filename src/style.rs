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

    /// The alpha component as an SVG opacity value (0.0–1.0).
    pub fn alpha(&self) -> f32 {
        self.a
    }
}

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
