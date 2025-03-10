fn main() {
    cxx_build::bridge("src/cxx/mod.rs") // returns a cc::Build
        .file("src/cxx/mod.cpp")
        .file("src/cxx/device.cpp")
        .std("c++17")
        .compile("cxxbridge-device");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/cxx/mod.rs");
    println!("cargo:rerun-if-changed=src/cxx/mod.h");
    println!("cargo:rerun-if-changed=src/cxx/mod.cpp");
    println!("cargo:rerun-if-changed=src/cxx/device.h");
    println!("cargo:rerun-if-changed=src/cxx/device.cpp");
}
