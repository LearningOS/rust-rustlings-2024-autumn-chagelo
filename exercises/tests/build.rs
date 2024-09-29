//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    // In tests7, we should set up an environment variable
    // called `TEST_FOO`. Print in the standard output to let
    // Cargo do it.
    let test_foo = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs(); // What's the use of this timestamp here?

    println!("cargo:rustc-env=TEST_FOO={}", test_foo);

    // In tests8, we should enable "pass" feature to make the
    // testcase return early. Fill in the command to tell
    // Cargo about that.
    // Enable the "pass" feature for the test case
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
