fn main() {
    println!("cargo:rerun-if-changed=src/qt_dialog.cpp");

    let qt = pkg_config::Config::new()
        .atleast_version("6.5")
        .probe("Qt6Widgets")
        .expect("Qt 6 Widgets development files are required (Arch package: qt6-base)");

    let mut build = cc::Build::new();
    build
        .cpp(true)
        .std("c++17")
        .file("src/qt_dialog.cpp")
        .warnings(true);

    for include in qt.include_paths {
        build.include(include);
    }

    build.compile("flufflinux_qt_dialog");
}
