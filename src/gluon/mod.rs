//! Gluon scripting bindings for hagoromo.
//!
//! Behind the optional `gluon` feature, this module exposes hagoromo's full
//! public API to embedded [Gluon](https://github.com/gluon-lang/gluon) scripts.
//!
//! # Quick start
//!
//! ```no_run
//! use gluon::{new_vm, ThreadExt};
//! let vm = new_vm();
//! hagoromo::gluon::register(&vm).unwrap();
//! vm.load_script("hagoromo", hagoromo::gluon::PRELUDE).unwrap();
//!
//! let src = r#"
//!     let { prim, (|>) } = import! hagoromo
//!     let { circle, color, fc, render_svg, render_options_default } = prim
//!     render_svg (circle 1.0 |> fc color.red) render_options_default
//! "#;
//! let (svg, _): (String, _) = vm.run_expr("user", src).unwrap();
//! println!("{svg}");
//! ```

pub mod userdata;
pub mod prim;

use gluon::Thread;

/// Source of the Gluon prelude that defines the operator sugar (`<>`, `|||`,
/// `===`, `#`). Load it with `vm.load_script("hagoromo", PRELUDE)` after
/// [`register`].
pub const PRELUDE: &str = include_str!("../../prelude/hagoromo.glu");

/// Register hagoromo's primitives module so that scripts can `import! hagoromo.prim`.
///
/// Also registers each userdata type with the VM. Call once per VM, before
/// loading the prelude or running user scripts.
pub fn register(vm: &Thread) -> gluon::Result<()> {
    // Register userdata types — name must match vm_type in the derive.
    vm.register_type::<userdata::GDiagram>("hagoromo.types.Diagram", &[])?;
    vm.register_type::<userdata::GTrail>("hagoromo.types.Trail", &[])?;
    vm.register_type::<userdata::GColor>("hagoromo.types.Color", &[])?;
    vm.register_type::<userdata::GMeasure>("hagoromo.types.Measure", &[])?;
    vm.register_type::<userdata::GBoundingBox>("hagoromo.types.BoundingBox", &[])?;
    vm.register_type::<userdata::GRect>("hagoromo.types.Rect", &[])?;
    vm.register_type::<userdata::GPoint>("hagoromo.types.Point", &[])?;
    vm.register_type::<userdata::GVec2>("hagoromo.types.Vec2", &[])?;
    vm.register_type::<userdata::GCubicSpline>("hagoromo.types.CubicSpline", &[])?;
    vm.register_type::<userdata::GRadialGradient>("hagoromo.types.RadialGradient", &[])?;
    vm.register_type::<userdata::GGradientStop>("hagoromo.types.GradientStop", &[])?;
    vm.register_type::<userdata::GRenderOptions>("hagoromo.types.RenderOptions", &[])?;

    gluon::import::add_extern_module(vm, "hagoromo.prim", prim::load);
    Ok(())
}
