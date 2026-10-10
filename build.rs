//! # Kernel Build Script (FFI Integration)
//!
//! This Cargo build script (`build.rs`) automates the linking process for 
//! the precompiled SPARK/Ada subsystem objects (`time.o`) into the freestanding 
//! RISC-V binary, ensuring proper dependency tracking and linker arguments.

use std::env;
use std::path::PathBuf;

/// Entry point for the build script.
/// 
/// Performs the following build configuration steps:
/// 1. Resolves the absolute path to the package manifest directory.
/// 2. Configures Cargo to rerun this build script if the precompiled Ada object file changes.
/// 3. Injects the path of the Ada object (`time.o`) directly into the linker arguments 
///    via cargo instructions (`cargo:rustc-link-arg`).
fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let ada_obj_dir = manifest_dir.join("ada/time/obj");

    // Inform Cargo to re-run the build script only if the SPARK/Ada object file is modified.
    println!("cargo:rerun-if-changed=ada/time/obj/time.o");

    // Pass the precompiled Ada object file directly to the linker for FFI resolution.
    println!("cargo:rustc-link-arg={}", ada_obj_dir.join("time.o").display());
}