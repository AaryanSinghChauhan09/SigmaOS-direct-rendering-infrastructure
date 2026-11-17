//! SigmaOS-direct-rendering-infrastructure — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_direct_rendering_infrastructure_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-direct-rendering-infrastructure: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-direct-rendering-infrastructure: ok");
}
