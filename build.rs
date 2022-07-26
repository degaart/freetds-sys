use std::process::Command;
use std::{env, path::PathBuf};
use std::fs;

fn main() {
    println!("cargo:rerun-if-changed=freetds-1.3.12.tar.gz");
    println!("cargo:rerun-if-changed=freetds.h");

    let src_dir = env::current_dir().unwrap();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let prefix = out_dir.join("prefix");

    let status = Command::new("tar")
        .arg("-zxf")
        .arg(src_dir.join("freetds-1.3.12.tar.gz"))
        .current_dir(&out_dir)
        .status()
        .expect("extract failed");
    if !status.success() {
        panic!("extract failed");
    }

    let build_dir = out_dir.join("build");
    if !build_dir.exists() {
        fs::create_dir(&build_dir).unwrap();
    }

    let status = Command::new(out_dir.join("freetds-1.3.12").join("configure"))
        .arg("--disable-dependency-tracking")
        .arg("--disable-shared")
        .arg("--disable-sspi")
        .arg("--disable-odbc")
        .arg("--disable-apps")
        .arg("--disable-server")
        .arg("--disable-pool")
        .arg(&format!("--prefix={}", prefix.display()))
        .arg("--enable-sybase-compat")
        .current_dir(&build_dir)
        .status()
        .expect("configure failed");
    if !status.success() {
        panic!("configure failed");
    }

    let status = make_cmd::gnu_make()
        .arg(&format!("-j{}", num_cpus::get()))
        .current_dir(&build_dir)
        .status()
        .expect("make failed");
    if !status.success() {
        panic!("make failed");
    }

    let status = make_cmd::gnu_make()
        .arg("install")
        .current_dir(&build_dir)
        .status()
        .expect("make install failed");
    if !status.success() {
        panic!("make install failed");
    }

    let bindings = bindgen::builder()
        .header("freetds.h")
        .clang_arg(format!("-I{}", prefix.join("include").display()))
        .layout_tests(false)
        .default_macro_constant_type(bindgen::MacroTypeVariation::Signed)
        .ctypes_prefix("libc")
        .allowlist_function("cs_.*|ct_.*")
        .allowlist_var("MSDBLIB|CTLIB|CS_.*|BLK_.*")
        .allowlist_type("CS_.*")
        .default_macro_constant_type(bindgen::MacroTypeVariation::Signed)
        .derive_default(true)
        .derive_copy(true)
        .derive_debug(true)
        .derive_eq(true)
        .generate()
        .expect("bindgen failed");
    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("bindgen failed");

    println!("cargo:rustc-link-search={}", prefix.join("lib").display());
    println!("cargo:rustc-link-lib=ct");
    println!("cargo:rustc-link-lib=sybdb");

    println!("cargo:root={}", prefix.display());
    println!("cargo:include={}", prefix.join("include").display());
    println!("cargo:libdir={}", prefix.join("lib").display());
}
