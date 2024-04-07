use bindgen::Builder;
use std::env;
use std::path::PathBuf;
use std::process::Command;

const VERSION: &str = "2.15";

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let build_parent_dir = out_path.join(format!("gperftools-{VERSION}"));
    let _ = std::fs::remove_dir_all(build_parent_dir.as_path());

    let output = Command::new("curl")
        .arg("-sLo")
        .arg(format!("gperftools-{VERSION}.tar.gz"))
        .arg(format!("https://github.com/gperftools/gperftools/releases/download/gperftools-{VERSION}/gperftools-{VERSION}.tar.gz"))
        .current_dir(out_path.as_path())
        .output()
        .unwrap();
    if !output.status.success() {
        panic!("unable to download gperftools")
    }

    let output = Command::new("tar")
        .arg("-xzf")
        .arg(format!("gperftools-{VERSION}.tar.gz"))
        .current_dir(out_path.as_path())
        .output()
        .unwrap();
    if !output.status.success() {
        panic!("unable to extract gperftools")
    }

    let output = Command::new("sh")
        .arg("configure")
        .arg("--with-tcmalloc-pagesize=32")
        .arg("--enable-static")
        .arg("--disable-shared")
        .arg("--disable-heap-checker")
        .arg("--disable-debugalloc")
        .arg("--disable-cpu-profiler")
        .current_dir(build_parent_dir.as_path())
        .output()
        .unwrap();
    if !output.status.success() {
        panic!("unable to configure gperftools")
    }

    let output = Command::new("make")
        .arg("-j")
        .arg(num_cpus::get().to_string())
        .current_dir(build_parent_dir.as_path())
        .output()
        .unwrap();
    if !output.status.success() {
        panic!("unable to make gperftools")
    }

    let bindings = Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I/{}/src/gperftools", build_parent_dir.display()))
        .generate()
        .expect("unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("unable to write bindings");

    println!(
        "cargo:rustc-link-search={}/.libs",
        build_parent_dir.display()
    );
    println!("cargo:rustc-link-lib=static=tcmalloc");
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-lib=unwind");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/lib.rs");
}
