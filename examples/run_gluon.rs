//! Run a Gluon `.glu` script and print its SVG result to stdout.
//!
//! Usage: `cargo run --features gluon --example run_gluon -- <script.glu>`

use gluon::{new_vm, ThreadExt};

fn main() {
    let path = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("usage: run_gluon <script.glu>");
        std::process::exit(2);
    });

    let vm = new_vm();
    vm.get_database_mut().set_implicit_prelude(false);
    hagoromo::gluon::register(&vm).expect("register hagoromo bindings");
    vm.load_script("hagoromo", hagoromo::gluon::PRELUDE)
        .expect("load hagoromo prelude");

    let src = std::fs::read_to_string(&path).expect("read script");
    let (svg, _) = vm
        .run_expr::<String>("user", &src)
        .expect("run script");
    print!("{svg}");
}
