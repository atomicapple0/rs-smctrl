fn main() {
    println!("cargo:rustc-link-lib=dylib=cuda");

    let src = ["src/libsmctrl.c"];
    let mut builder = cc::Build::new();
    let build = builder
        .files(src.iter())
        .include("/usr/local/cuda/include")
        .flag("-Wno-unused-parameter")
        .flag("-Wno-parentheses")
        .flag("-Wno-sign-compare");
    build.compile("libsmctrl");
}
