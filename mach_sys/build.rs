#[cfg(all(
    feature = "build_bindings",
    any(target_os = "macos", target_os = "ios")
))]
fn main() {
    use std::{env, path::PathBuf};
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Failed to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

#[cfg(not(any(target_os = "macos", target_os = "ios")))]
fn main() {
    panic!("This crate only targets macOS and iOS platforms.");
}

#[cfg(all(
    not(feature = "build_bindings"),
    any(target_os = "macos", target_os = "ios")
))]
fn main() {}
