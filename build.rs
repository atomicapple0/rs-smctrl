use std::{env, path::Path, process::Command};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    // Note that there are a number of downsides to this approach
    // See: https://doc.rust-lang.org/cargo/reference/build-script-examples.html
    Command::new("gcc")
        .args(&[
            "src/libsmctrl.c",
            "-c",
            "-fPIC",
            "-lcuda",
            "-I/usr/local/cuda/include",
            "-o",
        ])
        .arg(&format!("{}/libsmctrl.o", out_dir))
        .status()
        .unwrap();
    Command::new("ar")
        .args(&["rcs", "libsmctrl.a", "libsmctrl.o"])
        .current_dir(&Path::new(&out_dir))
        .status()
        .unwrap();
    println!("{}", out_dir);

    println!("cargo:rustc-link-search={}", out_dir);
}
