use cc::Build;

fn main() {
    let mut builder = Build::new();

    builder
        .define("UNICODE", "1")
        .file("src/source.cpp")
        .compile("auto_elevate");
}
