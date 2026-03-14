//! Cubic spline: parametric curve interpolating through a set of points.

use kurbo::{CubicBez, ParamCurve, ParamCurveDeriv, Point, Vec2};

/// A cubic spline that interpolates through a set of points.
///
/// Created by [`cubic_spline`]. Supports parametric evaluation via
/// [`at_param`][CubicSpline::at_param], [`tangent_at_param`][CubicSpline::tangent_at_param],
/// and [`normal_at_param`][CubicSpline::normal_at_param].
///
/// **Parameter domain**: `t ∈ [0, 1]` — 0 is the start, 1 is the end of the spline.
///
/// Render it as a [`Diagram`][crate::Diagram] with [`stroke_spline`][crate::stroke_spline].
pub struct CubicSpline {
    pub(crate) segments: Vec<CubicBez>,
}

impl CubicSpline {
    /// Evaluate the position on the spline at parameter `t` ∈ [0, 1].
    pub fn at_param(&self, t: f64) -> Point {
        let (idx, lt) = self.seg_and_t(t);
        self.segments[idx].eval(lt)
    }

    /// Evaluate the (unnormalized) tangent vector at parameter `t` ∈ [0, 1].
    ///
    /// The magnitude reflects the Bézier parameterization speed. Divide by `.length()`
    /// for a unit tangent.
    pub fn tangent_at_param(&self, t: f64) -> Vec2 {
        let (idx, lt) = self.seg_and_t(t);
        let dp: Point = self.segments[idx].deriv().eval(lt);
        Vec2::new(dp.x, dp.y)
    }

    /// Evaluate the normal vector (90° CCW from the tangent) at parameter `t` ∈ [0, 1].
    ///
    /// Same magnitude as [`tangent_at_param`][CubicSpline::tangent_at_param].
    pub fn normal_at_param(&self, t: f64) -> Vec2 {
        let tv = self.tangent_at_param(t);
        Vec2::new(-tv.y, tv.x)
    }

    fn seg_and_t(&self, t: f64) -> (usize, f64) {
        let n = self.segments.len();
        let scaled = (t * n as f64).clamp(0.0, n as f64);
        let idx = (scaled.floor() as usize).min(n - 1);
        // Subtract idx (not floor) so that t=1.0 gives local_t=1.0 on the last segment.
        let local_t = (scaled - idx as f64).clamp(0.0, 1.0);
        (idx, local_t)
    }
}

/// Construct a natural cubic spline interpolating the given points.
///
/// Uses zero-second-derivative boundary conditions (natural spline), giving C²
/// continuity at interior knots. Corresponds to Haskell diagrams' `cubicSpline False pts`.
///
/// # Panics
/// Panics if `pts` has fewer than 2 points.
pub fn cubic_spline(pts: &[Point]) -> CubicSpline {
    assert!(pts.len() >= 2, "cubic_spline requires at least 2 points");
    let n = pts.len() - 1; // number of segments

    let xs: Vec<f64> = pts.iter().map(|p| p.x).collect();
    let ys: Vec<f64> = pts.iter().map(|p| p.y).collect();

    // Solve the tridiagonal system for second derivatives (moments) at each knot.
    // Natural boundary: M[0] = M[n] = 0.
    let mx = natural_moments(&xs);
    let my = natural_moments(&ys);

    // Convert moments → cubic Bézier control points for each segment.
    // For segment i (uniform h=1), the Bézier are:
    //   p0 = pts[i],  p3 = pts[i+1]
    //   p1 = p0 + (p3−p0)/3 − (2·M[i] + M[i+1]) / 18
    //   p2 = p3 − (p3−p0)/3 − (M[i] + 2·M[i+1]) / 18
    let segments = (0..n)
        .map(|i| {
            let p0 = pts[i];
            let p3 = pts[i + 1];
            let x1 = p0.x + (p3.x - p0.x) / 3.0 - (2.0 * mx[i] + mx[i + 1]) / 18.0;
            let y1 = p0.y + (p3.y - p0.y) / 3.0 - (2.0 * my[i] + my[i + 1]) / 18.0;
            let x2 = p3.x - (p3.x - p0.x) / 3.0 - (mx[i] + 2.0 * mx[i + 1]) / 18.0;
            let y2 = p3.y - (p3.y - p0.y) / 3.0 - (my[i] + 2.0 * my[i + 1]) / 18.0;
            CubicBez::new(p0, Point::new(x1, y1), Point::new(x2, y2), p3)
        })
        .collect();

    CubicSpline { segments }
}

/// Solve for the interior moments of a natural cubic spline with uniform knot spacing.
///
/// Returns a vector of n+1 moments (M[0]=M[n]=0; M[1]..M[n-1] are solved via
/// the Thomas algorithm on the tridiagonal system with diagonal 4, off-diagonals 1).
fn natural_moments(vals: &[f64]) -> Vec<f64> {
    let n = vals.len() - 1;
    let mut m = vec![0.0f64; n + 1];

    if n < 2 {
        return m; // no interior knots
    }

    let size = n - 1;
    let mut diag = vec![4.0f64; size];
    let mut rhs: Vec<f64> = (1..n)
        .map(|i| 6.0 * (vals[i + 1] - 2.0 * vals[i] + vals[i - 1]))
        .collect();

    // Forward sweep (Thomas algorithm)
    for i in 1..size {
        let f = 1.0 / diag[i - 1];
        diag[i] -= f;
        rhs[i] -= f * rhs[i - 1];
    }

    // Back substitution
    let mut sol = vec![0.0f64; size];
    sol[size - 1] = rhs[size - 1] / diag[size - 1];
    for i in (0..size - 1).rev() {
        sol[i] = (rhs[i] - sol[i + 1]) / diag[i];
    }

    for (i, &v) in sol.iter().enumerate() {
        m[i + 1] = v;
    }
    m
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn spline_passes_through_endpoints() {
        let pts = &[Point::new(0.0, 0.0), Point::new(1.0, 1.0), Point::new(2.0, 0.0)];
        let s = cubic_spline(pts);
        let p0 = s.at_param(0.0);
        let p1 = s.at_param(1.0);
        assert_relative_eq!(p0.x, 0.0, epsilon = 1e-9);
        assert_relative_eq!(p0.y, 0.0, epsilon = 1e-9);
        assert_relative_eq!(p1.x, 2.0, epsilon = 1e-9);
        assert_relative_eq!(p1.y, 0.0, epsilon = 1e-9);
    }

    #[test]
    fn normal_is_perpendicular_to_tangent() {
        let pts = &[Point::new(0.0, 0.0), Point::new(1.0, 0.5), Point::new(2.0, 0.0)];
        let s = cubic_spline(pts);
        let t = s.tangent_at_param(0.5);
        let n = s.normal_at_param(0.5);
        let dot = t.x * n.x + t.y * n.y;
        assert_relative_eq!(dot, 0.0, epsilon = 1e-9);
    }
}
