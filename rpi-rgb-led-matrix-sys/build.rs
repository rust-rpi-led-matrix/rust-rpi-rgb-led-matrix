//! Build script for `rpi-rgb-led-matrix-sys`
//!
//! This build script:
//! 0. checks if we're on a raspberry pi to make sure compilation has a chance of success
//! 1. copies our git submodule checkout of the C++ library to build artifacts
//! 2. builds the C++ library from there
//! 3. statically links against it

fn main() {
    // 0. To guess at if we're on the right platform, look for linux as the system & arm as the architecture
    // Note I'm checking HOST instead of TARGET since the C++ library depends on natively linking to some libraries
    //   that are only on rpis
    let host = std::env::var("HOST").unwrap();
    if !host.contains("arm") || !host.contains("linux") {
        eprintln!("rpi-rgb-led-matrix-sys detected you're likely not compiling on a raspberry pi");
        std::process::exit(-1);
    }

    // 1. copy our git submodule over to build artifacts so things like `cargo clean` work properly
    let target_dir = std::env::var("OUT_DIR").unwrap();
    let repo_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let cpp_lib_dir: std::path::PathBuf = [&repo_dir, "cpp-library"].iter().collect();
    let cpp_lib_out_dir: std::path::PathBuf = [&target_dir, "cpp-library"].iter().collect();
    println!("building from {}", cpp_lib_out_dir.display());
    copy_dir::copy_dir(&cpp_lib_dir, &cpp_lib_out_dir).unwrap();
    println!("cargo:rerun-if-changed={}", cpp_lib_dir.display());

    // 2. build the library
    let cpp_lib_lib_out_dir: std::path::PathBuf =
        [&cpp_lib_out_dir.to_str().unwrap(), "lib"].iter().collect();
    std::env::set_current_dir(&cpp_lib_lib_out_dir).unwrap();
    let status = std::process::Command::new("make")
        .status()
        .expect("process failed to execute");
    if !status.success() {
        panic!("failed to compile the C++ library");
    }

    // 3. link!
    println!(
        "cargo:rustc-link-search=native={}",
        cpp_lib_lib_out_dir.display()
    );
    println!("cargo:rustc-link-lib=static=rgbmatrix");
}
