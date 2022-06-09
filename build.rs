use std::process::Command;
use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=freetds-src");
    println!("cargo:rerun-if-changed=freetds.h");

    let src_dir = env::current_dir().unwrap();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let status = Command::new(src_dir.join("freetds-src").join("configure"))
        .arg("--disable-dependency-tracking")
        .arg("--disable-shared")
        .arg("--disable-sspi")
        .arg("--disable-odbc")
        .arg("--disable-apps")
        .arg("--disable-server")
        .arg("--disable-pool")
        /*.arg("--enable-msdblib")*/
        .arg("--enable-sybase-compat")
        .current_dir(&out_dir)
        .status()
        .expect("configure failed");
    if !status.success() {
        panic!("configure failed");
    }

    let status = make_cmd::gnu_make()
        .arg(&format!("-j{}", num_cpus::get()))
        .current_dir(&out_dir)
        .status()
        .expect("make failed");
    if !status.success() {
        panic!("make failed");
    }

    let bindings = bindgen::builder()
        .header("freetds.h")
        .clang_arg("-Ifreetds-src/include")
        .clang_arg(format!("-I{}", out_dir.join("include").display()))
        .layout_tests(false)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .default_macro_constant_type(bindgen::MacroTypeVariation::Signed)
        .ctypes_prefix("libc")
        .allowlist_function("cs_.*|ct_.*")
        .allowlist_var("MSDBLIB|CTLIB|CS_.*|BLK_.*")
        .generate()
        .expect("bindgen failed");
    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("bindgen failed");

    println!("cargo:rustc-link-search={}", out_dir.join("src/tds/.libs").display());
    println!("cargo:rustc-link-search={}", out_dir.join("src/ctlib/.libs").display());
    println!("cargo:rustc-link-search={}", out_dir.join("src/dblib/.libs").display());

    println!("cargo:rustc-link-lib=tds");
    println!("cargo:rustc-link-lib=ct");
    println!("cargo:rustc-link-lib=sybdb");
}
