use std::env;
use std::path::PathBuf;

fn main() {
    // Uses shared library files, automatically built by gradle.
    // Wish we could statically link this but unfortunately trying to build it results in a build failure,
    // and its not part of the usual build task so I'm guessing thats just a fun feature
    // #[cfg(target_os = "linux")]
    // let libdir_relative_path = format!(
    //     "../allwpilib/{}/build/libs/{}/shared/linuxx86-64/release",
    //     "ntcoreffi", "ntcoreffi"
    // );
    // let libdir_path = PathBuf::from(libdir_relative_path)
    //     .canonicalize()
    //     .expect("cannot canonicalize path");
    // println!("cargo:rustc-link-search={}", libdir_path.to_str().unwrap());

    let ntcoreffi_dir = PathBuf::from("ntcoreffi").canonicalize().expect("bad path");
    let ntcoreffi_dir_str = ntcoreffi_dir.to_str().unwrap();
    // Windows inserts a mysterious "\\?\" before the paths, which causes
    // clang to completely break. No idea why.
    #[cfg(target_os = "windows")]
    let ntcoreffi_dir_str = &ntcoreffi_dir_str[4..];
    #[cfg(target_os = "windows")]
    let dir = PathBuf::from(".").canonicalize().expect("bad path");
    #[cfg(target_os = "windows")]
    let dir_str = &dir.to_str().unwrap()[4..];

    println!("cargo:rustc-link-search={}", ntcoreffi_dir_str);

    println!("cargo:rustc-link-lib={}", "ntcoreffi");

    let headers_path = PathBuf::from("wrapper.h").canonicalize().expect("bad path");

    let headers_path_str = headers_path.to_str().expect("Path is not a valid string");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(headers_path_str)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // makes the <> includes work properly. (why would they do this?)
        .clang_args([
            "-I",
            ntcoreffi_dir_str,
            #[cfg(target_os = "windows")]
            "-I",
            #[cfg(target_os = "windows")]
            dir_str,
        ])
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
