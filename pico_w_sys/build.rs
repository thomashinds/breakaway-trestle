use std::iter::once;

fn main() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=../init/sdk_includes.h");

    let compile_args_file =
        std::fs::read_to_string("compile_flags.txt").expect("Couldn't find compile_flags.txt");
    let compile_args = compile_args_file.lines().chain(once(
        "-I/opt/homebrew/Cellar/arm-gcc-bin@12/12.2.Rel1/arm-none-eabi/include/",
    ));

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .clang_args(compile_args)
        .header("../init/sdk_includes.h")
        .ctypes_prefix("cty")
        .use_core()
        .layout_tests(false)
        .default_enum_style(bindgen::EnumVariation::NewType {
            is_bitfield: true,
            is_global: false,
        })
        .raw_line("#![no_std]")
        .raw_line("#![allow(non_upper_case_globals)]")
        .raw_line("#![allow(non_camel_case_types)]")
        .raw_line("#![allow(non_snake_case)]")
        .raw_line("#![allow(dead_code)]")
        // .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Need to make this filepath not hardcoded to my own directory
    bindings
        .write_to_file("/Users/thomas/Documents/peloton/picotors/pico_w_sys/src/lib.rs")
        .expect("Couldn't write bindings!");
}
