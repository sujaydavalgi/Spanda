//! build support for Spanda.
//!
extern crate napi_build;

fn main() {
    // Main.
    //
    // Parameters:
    // None.
    //
    // Returns:
    // Nothing.
    //
    // Options:
    // None.
    //
    // Example:
    // let result = spanda_node::build::main();

    // Produce setup as the result.
    napi_build::setup();
}
