extern crate cbindgen;

use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::Builder::new()
        .with_crate(&crate_dir)
        .with_parse_deps(true)
        .with_parse_include(&["bochscpu"])
        .with_header("#pragma once")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("bochscpu.hpp");

    cbindgen::Builder::new()
        .with_crate(&crate_dir)
        .with_parse_deps(true)
        .with_parse_include(&["bochscpu"])
        .with_language(cbindgen::Language::C)
        .with_header("#pragma once")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("bochscpu.h");
}
