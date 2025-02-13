//! Build script for `rpi-led-matrix-sys`
//!
//! This build script:
//! 0. checks if we're on a raspberry pi to make sure compilation has a chance of success
//! 1. copies our git submodule checkout of the C++ library to build artifacts
//! 2. builds the C++ library from there
//! 3. statically links against it
use std::process::Command;

fn main() {
    // Early out if we're stubbing the C api ourselves
    if std::env::var("CARGO_FEATURE_C_STUBS").is_ok() {
        std::process::exit(0);
    }

    // link to c++ std library as the c++ library depends on it
    if std::env::var("CARGO_FEATURE_STDCPP_STATIC_LINK").is_ok() {
        // statically link as requested
        println!("cargo:rustc-link-lib=static=stdc++");
    } else {
        // default path: dynamically link
        println!("cargo:rustc-flags=-l dylib=stdc++");
    }

    // 0. To guess at if we're targetting the right platform, look for linux as the system & arm as the architecture
    let target = std::env::var("TARGET").unwrap();
    if !(target.contains("arm") || target.contains("aarch")) || !target.contains("linux") {
        eprintln!("rpi-led-matrix-sys detected you're likely not compiling for a raspberry pi");
        std::process::exit(-1);
    }

    // 1. copy our git submodule over to build artifacts so things like `cargo clean` work properly
    let target_dir = std::env::var("OUT_DIR").unwrap();
    let repo_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let cpp_lib_dir: std::path::PathBuf = [&repo_dir, "cpp-library"].iter().collect();
    let cpp_lib_out_dir: std::path::PathBuf = [&target_dir, "cpp-library"].iter().collect();

    // Make sure our git submodule is checked out and up to date, if we are local/have git
    if Command::new("git")
        .arg("status")
        .status()
        .expect("git status failed")
        .success()
        && !Command::new("git")
            .arg("submodule")
            .arg("update")
            .arg("--init")
            .status()
            .expect("process failed to execute")
            .success()
    {
        println!("cargo:warning=failed to checkout/update the C++ library git submodule");
    }

    // delete our output git directory, if it exists, then copy the git repo over
    std::fs::remove_dir_all(&cpp_lib_out_dir).ok();
    copy_dir::copy_dir(&cpp_lib_dir, &cpp_lib_out_dir).unwrap();
    println!("cargo:rerun-if-changed={}", cpp_lib_dir.display());

    // 2. build the library. We assume you have the tools necessary to build the library,
    //    which I think are available by default on all pis
    let cpp_lib_lib_out_dir: std::path::PathBuf =
        [cpp_lib_out_dir.to_str().unwrap(), "lib"].iter().collect();
    std::env::set_current_dir(&cpp_lib_lib_out_dir).unwrap();
    println!("building from {}", cpp_lib_out_dir.display());
    let cc_path = cc::Build::new().get_compiler();
    let cxx_path = cc::Build::new().cpp(true).get_compiler();
    let status = Command::new("make")
        .args([
            format!("CC={}", cc_path.path().to_str().unwrap()),
            format!("CXX={}", cxx_path.path().to_str().unwrap()),
        ])
        .status()
        .expect("process failed to execute");
    assert!(status.success(), "failed to compile the C++ library");

    // 2.1 rename the library produced to avoid ambiguity with global variants.
    let cpp_lib_lib_out_file: std::path::PathBuf =
        [cpp_lib_out_dir.to_str().unwrap(), "lib", "librgbmatrix.a"]
            .iter()
            .collect();
    let cpp_lib_lib_out_file_rename: std::path::PathBuf = [
        cpp_lib_out_dir.to_str().unwrap(),
        "lib",
        "librgbmatrixsys.a",
    ]
    .iter()
    .collect();
    println!(
        "renaming library from {:?} to {:?}",
        &cpp_lib_lib_out_file, &cpp_lib_lib_out_file_rename
    );
    std::fs::rename(&cpp_lib_lib_out_file, &cpp_lib_lib_out_file_rename).unwrap();

    // 3. link!
    println!(
        "cargo:rustc-link-search=native={}",
        cpp_lib_lib_out_dir.display()
    );
    println!("cargo:rustc-link-lib=static=rgbmatrixsys");
}
