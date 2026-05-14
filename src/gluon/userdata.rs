//! Newtype wrappers around hagoromo public types so they can be exposed to
//! Gluon as opaque userdata.
//!
//! Each wrapper derives `Userdata + Trace + Clone + Debug + VmType` and
//! adopts the `gluon_userdata(clone)` strategy — gluon will clone the
//! wrapper when transferring values across the FFI boundary. Hagoromo's
//! types are all `Arc`-backed or pure value types, so cloning is cheap.

use std::sync::Arc;

use gluon_codegen::{Trace, Userdata, VmType};
use kurbo::{Point, Rect, Vec2};

use crate::{
    style::{Color, GradientStop, Measure, RadialGradient},
    BoundingBox, CubicSpline, Diagram, RenderOptions, Trail,
};

// ── Diagram ──────────────────────────────────────────────────────────────────

#[derive(Userdata, Trace, Clone, Debug, VmType)]
#[gluon_userdata(clone)]
#[gluon_trace(skip)]
#[gluon(vm_type = "hagoromo.types.Diagram")]
pub struct GDiagram(pub Diagram);

impl From<Diagram> for GDiagram {
    fn from(d: Diagram) -> Self { GDiagram(d) }
}

// ── Trail ────────────────────────────────────────────────────────────────────

#[derive(Userdata, Trace, Clone, Debug, VmType)]
#[gluon_userdata(clone)]
#[gluon_trace(skip)]
#[gluon(vm_type = "hagoromo.types.Trail")]
pub struct GTrail(pub Trail);

impl From<Trail> for GTrail {
    fn from(t: Trail) -> Self { GTrail(t) }
}

// ── Color ────────────────────────────────────────────────────────────────────

#[derive(Userdata, Trace, Clone, Debug, VmType)]
#[gluon_userdata(clone)]
#[gluon_trace(skip)]
#[gluon(vm_type = "hagoromo.types.Color")]
pub struct GColor(pub Color);

impl From<Color> for GColor {
    fn from(c: Color) -> Self { GColor(c) }
}

// ── Measure ──────────────────────────────────────────────────────────────────

#[derive(Userdata, Trace, Clone, Debug, VmType)]
#[gluon_userdata(clone)]
#[gluon_trace(skip)]
#[gluon(vm_type = "hagoromo.types.Measure")]
pub struct GMeasure(pub Measure);

impl From<Measure> for GMeasure {
    fn from(m: Measure) -> Self { GMeasure(m) }
}

// ── BoundingBox ──────────────────────────────────────────────────────────────

#[derive(Userdata, Trace, Clone, Debug, VmType)]
#[gluon_userdata(clone)]
#[gluon_trace(skip)]
#[gluon(vm_type = "hagoromo.types.BoundingBox")]
pub struct GBoundingBox(pub BoundingBox);

impl From<BoundingBox> for GBoundingBox {
    fn from(b: BoundingBox) -> Self { GBoundingBox(b) }
}

// ── Rect ─────────────────────────────────────────────────────────────────────

#[derive(Userdata, Trace, Clone, Debug, VmType)]
#[gluon_userdata(clone)]
#[gluon_trace(skip)]
#[gluon(vm_type = "hagoromo.types.Rect")]
pub struct GRect(pub Rect);

impl From<Rect> for GRect {
    fn from(r: Rect) -> Self { GRect(r) }
}

// ── Point ────────────────────────────────────────────────────────────────────

#[derive(Userdata, Trace, Clone, Debug, VmType)]
#[gluon_userdata(clone)]
#[gluon_trace(skip)]
#[gluon(vm_type = "hagoromo.types.Point")]
pub struct GPoint(pub Point);

impl From<Point> for GPoint {
    fn from(p: Point) -> Self { GPoint(p) }
}

// ── Vec2 ─────────────────────────────────────────────────────────────────────

#[derive(Userdata, Trace, Clone, Debug, VmType)]
#[gluon_userdata(clone)]
#[gluon_trace(skip)]
#[gluon(vm_type = "hagoromo.types.Vec2")]
pub struct GVec2(pub Vec2);

impl From<Vec2> for GVec2 {
    fn from(v: Vec2) -> Self { GVec2(v) }
}

// ── CubicSpline ──────────────────────────────────────────────────────────────
//
// CubicSpline doesn't impl Clone or Debug, so we wrap it in an Arc and
// provide a manual Debug impl. The Arc gives us cheap Clone for the wrapper.

#[derive(Userdata, Trace, Clone, VmType)]
#[gluon_userdata(clone)]
#[gluon_trace(skip)]
#[gluon(vm_type = "hagoromo.types.CubicSpline")]
pub struct GCubicSpline(pub Arc<CubicSpline>);

impl std::fmt::Debug for GCubicSpline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CubicSpline").finish_non_exhaustive()
    }
}

impl From<CubicSpline> for GCubicSpline {
    fn from(s: CubicSpline) -> Self { GCubicSpline(Arc::new(s)) }
}

// ── RadialGradient ───────────────────────────────────────────────────────────

#[derive(Userdata, Trace, Clone, Debug, VmType)]
#[gluon_userdata(clone)]
#[gluon_trace(skip)]
#[gluon(vm_type = "hagoromo.types.RadialGradient")]
pub struct GRadialGradient(pub RadialGradient);

impl From<RadialGradient> for GRadialGradient {
    fn from(g: RadialGradient) -> Self { GRadialGradient(g) }
}

// ── GradientStop ─────────────────────────────────────────────────────────────

#[derive(Userdata, Trace, Clone, Debug, VmType)]
#[gluon_userdata(clone)]
#[gluon_trace(skip)]
#[gluon(vm_type = "hagoromo.types.GradientStop")]
pub struct GGradientStop(pub GradientStop);

impl From<GradientStop> for GGradientStop {
    fn from(s: GradientStop) -> Self { GGradientStop(s) }
}

// ── RenderOptions ────────────────────────────────────────────────────────────

#[derive(Userdata, Trace, Clone, Debug, VmType)]
#[gluon_userdata(clone)]
#[gluon_trace(skip)]
#[gluon(vm_type = "hagoromo.types.RenderOptions")]
pub struct GRenderOptions(pub RenderOptions);

impl From<RenderOptions> for GRenderOptions {
    fn from(o: RenderOptions) -> Self { GRenderOptions(o) }
}
