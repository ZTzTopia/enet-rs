use cmake::Config;
use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    let is_debug = env::var("DEBUG").unwrap() == "true";

    let dst = Config::new("vendor/enet")
        .build_target("enet")
        .build();

    if target.contains("windows") {
        if is_debug {
            println!("cargo:rustc-link-search=native={}/build/Debug", dst.display());
        } else {
            println!("cargo:rustc-link-search=native={}/build/Release", dst.display());
        }

        println!("cargo:rustc-link-lib=dylib=winmm");
    } else {
        println!("cargo:rustc-link-search=native={}/build", dst.display());
    }

    println!("cargo:rustc-link-lib=static=enet");
}