use std::env;
use std::fs;
use std::path::{Path, PathBuf};

/// Resolves a directory from an env var override, falling back to a vendored path.
/// Panics with an actionable message if neither exists.
fn resolve_dir(env_var: &str, vendored_fallback: &Path, what: &str) -> PathBuf {
    if let Ok(dir) = env::var(env_var) {
        let dir = PathBuf::from(dir);
        if !dir.is_dir() {
            panic!(
                "{env_var} is set to `{}`, but that directory doesn't exist. \
                 It should contain {what}.",
                dir.display()
            );
        }
        return dir;
    }

    if vendored_fallback.is_dir() {
        return vendored_fallback.to_path_buf();
    }

    panic!(
        "Could not find {what}. Set the `{env_var}` environment variable to a directory \
         containing it, or restore the vendored copy at `{}`.",
        vendored_fallback.display()
    );
}

/// Walks up from OUT_DIR (`target/<profile>/build/<pkg>-<hash>/out`) to `target/<profile>`,
/// where cargo places the final build artifacts.
fn target_profile_dir(out_dir: &Path) -> PathBuf {
    out_dir
        .ancestors()
        .nth(3)
        .expect("OUT_DIR had fewer ancestors than expected")
        .to_path_buf()
}

fn copy_dlls(src_dir: &Path, dest_dir: &Path) {
    let entries = match fs::read_dir(src_dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext.eq_ignore_ascii_case("dll")) {
            let dest = dest_dir.join(path.file_name().unwrap());
            if let Err(e) = fs::copy(&path, &dest) {
                println!(
                    "cargo:warning=failed to copy {} to {}: {e}",
                    path.display(),
                    dest.display()
                );
            }
        }
    }
}

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let vendor_dir = manifest_dir.join("vendor/e384c");

    let include_dir = resolve_dir(
        "E384C_INCLUDE_DIR",
        &vendor_dir.join("include"),
        "e384c.h",
    );
    let lib_dir = resolve_dir("E384C_LIB_DIR", &vendor_dir.join("lib"), "e384c.lib");

    println!("cargo:rustc-link-search={}", lib_dir.display());
    println!("cargo:rustc-link-lib=e384c");

    if cfg!(feature = "bundled") {
        let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
        let dest_dir = target_profile_dir(&out_dir);

        // e384c.dll itself: redistributable, vendored alongside the import lib/header.
        copy_dlls(&vendor_dir.join("bin"), &dest_dir);

        // e384c's own third-party runtime deps: not vendored (unclear licensing),
        // must be supplied externally.
        match env::var("E384C_DLL_DIR") {
            Ok(dll_dir) => copy_dlls(Path::new(&dll_dir), &dest_dir),
            Err(_) => {
                println!(
                    "cargo:warning=E384C_DLL_DIR is not set — if e384c.dll's own runtime \
                     dependencies aren't already on PATH, set E384C_DLL_DIR to a directory \
                     containing them so build.rs can copy them next to the build output."
                );
            }
        }
    }

    let header = include_dir.join("e384c.h");

    let bindings = bindgen::Builder::default()
        .header(header.to_string_lossy())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Needed so plain-data structs (E384Measurement, E384RangedMeasurement, ...)
        // can be zero-initialized by the two-call size/fill helpers.
        .derive_default(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
