// Integration test for SigmaOS-direct-rendering-infrastructure
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use direct_rendering_infrastructure as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
