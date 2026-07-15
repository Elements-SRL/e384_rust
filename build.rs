use std::env;
use std::path::PathBuf;

/// Resolves a directory from a required env var. Panics with an actionable message if
/// the var is unset or doesn't point at a directory. No vendored fallback: every
/// directory this crate links against must be pointed to explicitly.
fn resolve_dir(env_var: &str, what: &str) -> PathBuf {
    let dir = env::var(env_var).unwrap_or_else(|_| {
        panic!("The `{env_var}` environment variable is not set. Set it to a directory containing {what}.")
    });
    let dir = PathBuf::from(dir);
    if !dir.is_dir() {
        panic!(
            "{env_var} is set to `{}`, but that directory doesn't exist. \
             It should contain {what}.",
            dir.display()
        );
    }
    dir
}

fn main() {
    let include_dir = resolve_dir("E384C_INCLUDE_DIR", "e384c.h");
    let lib_dir = resolve_dir("E384C_LIB_DIR", "e384c.lib");

    // `e384c` is now a STATIC lib. Its .lib holds real objects and leaves the
    // e384commlib + vendor-SDK symbols unresolved, so we must name every vendor
    // import lib on the final link line here.
    //
    // -------------------------------------------------------------------------
    // WHERE TO PUT THINGS (read this if a link step fails with LNK2019):
    //
    //   E384C_INCLUDE_DIR      -> directory containing `e384c.h`
    //   E384C_LIB_DIR          -> directory containing `e384c.lib`
    //   E384COMMLIB_LIB_DIR    -> directory containing `e384commlib.lib`
    //                             (the C++ core, static — no runtime DLL)
    //   FTD2XX_LIB_DIR         -> directory containing `ftd2xx.lib`
    //                             (FTDI D2XX import stub -> FTD2XX.dll at runtime)
    //   OKFRONTPANEL_LIB_DIR   -> directory containing `okFrontPanel.lib`
    //                             (Opal Kelly import stub -> okFrontPanel.dll at runtime)
    //   MPSSE_LIB_DIR          -> directory containing `MPSSE.lib`
    //                             (FTDI MPSSE import stub -> MPSSE.dll at runtime)
    //
    //   None of the vars above have a fallback or default — nothing is vendored
    //   in this repo, so every one of them must be set explicitly. Put the
    //   matching runtime DLLs (FTD2XX.dll, MPSSE.dll, okFrontPanel.dll) next to
    //   the final .exe yourself — this crate no longer copies them.
    // -------------------------------------------------------------------------

    println!("cargo:rustc-link-search={}", lib_dir.display()); // holds e384c.lib
    println!("cargo:rustc-link-lib=static=e384c");

    let e384commlib_dir = resolve_dir("E384COMMLIB_LIB_DIR", "e384commlib.lib");
    println!("cargo:rustc-link-search={}", e384commlib_dir.display());
    println!("cargo:rustc-link-lib=static=e384commlib");

    let ftd2xx_dir = resolve_dir("FTD2XX_LIB_DIR", "ftd2xx.lib");
    println!("cargo:rustc-link-search={}", ftd2xx_dir.display());
    println!("cargo:rustc-link-lib=ftd2xx"); // import stub -> FTD2XX.dll at runtime

    let okfrontpanel_dir = resolve_dir("OKFRONTPANEL_LIB_DIR", "okFrontPanel.lib");
    println!("cargo:rustc-link-search={}", okfrontpanel_dir.display());
    println!("cargo:rustc-link-lib=okFrontPanel"); // import stub -> okFrontPanel.dll at runtime

    let mpsse_dir = resolve_dir("MPSSE_LIB_DIR", "MPSSE.lib");
    println!("cargo:rustc-link-search={}", mpsse_dir.display());
    println!("cargo:rustc-link-lib=MPSSE"); // import stub -> MPSSE.dll at runtime

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
