use std::env;
use std::path::PathBuf;
use std::process::{Command, Output};

fn run(command: &mut Command, description: &str) -> Output {
    let output = command
        .output()
        .unwrap_or_else(|error| panic!("Could not run {description}: {error}"));

    if !output.status.success() {
        panic!(
            "{description} failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    output
}

fn pkg_config(arguments: &[&str]) -> Vec<String> {
    let output = run(
        Command::new("pkg-config").args(arguments).arg("Qt6Widgets"),
        "pkg-config for Qt 6 Widgets",
    );

    String::from_utf8(output.stdout)
        .expect("pkg-config returned non-UTF-8 output")
        .split_whitespace()
        .map(str::to_owned)
        .collect()
}

fn main() {
    println!("cargo:rerun-if-changed=src/qt_dialog.cpp");

    let output_directory =
        PathBuf::from(env::var_os("OUT_DIR").expect("Cargo did not set OUT_DIR"));
    let object_file = output_directory.join("qt_dialog.o");
    let bridge_library = output_directory.join("libflufflinux_qt_dialog.a");

    let mut compiler = Command::new("c++");
    compiler
        .args(["-std=c++17", "-fPIC", "-c", "src/qt_dialog.cpp", "-o"])
        .arg(&object_file)
        .args(pkg_config(&["--cflags"]));
    run(&mut compiler, "the system C++ compiler");

    let mut archiver = Command::new("ar");
    archiver
        .args(["crs"])
        .arg(&bridge_library)
        .arg(&object_file);
    run(&mut archiver, "the system archiver");

    println!(
        "cargo:rustc-link-search=native={}",
        output_directory.display()
    );
    println!("cargo:rustc-link-lib=static=flufflinux_qt_dialog");
    println!("cargo:rustc-link-lib=stdc++");

    for flag in pkg_config(&["--libs"]) {
        if let Some(path) = flag.strip_prefix("-L") {
            println!("cargo:rustc-link-search=native={path}");
        } else if let Some(library) = flag.strip_prefix("-l") {
            println!("cargo:rustc-link-lib={library}");
        } else {
            println!("cargo:rustc-link-arg={flag}");
        }
    }
}
