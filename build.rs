fn main() {
    let src = ["src/libsmctrl.c"];
    let mut builder = cc::Build::new();
    let build = builder
        .files(src.iter())
        .include("include")
        .flag("-Wno-unused-parameter")
        .flag("-Wno-parentheses")
        .flag("-Wno-sign-compare")
        .define("USE_ZLIB", None);
    build.compile("libsmctrl");
}
