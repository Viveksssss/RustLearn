use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    // 编译 hello.c
    cc::Build::new()
        .file("src/hello.c")
        .out_dir(&out_dir)
        .compile("hello");

    // 添加库搜索路径
    println!("cargo:rustc-link-search=native={}", out_dir);

    // 链接顺序很重要：先链接 hello，再链接 zlib
    println!("cargo:rustc-link-lib=static=hello");
    println!("cargo:rustc-link-lib=z");

    // 如果 zlib 在系统路径中，可能需要添加搜索路径
    // println!("cargo:rustc-link-search=native=/usr/lib/x86_64-linux-gnu");

    println!("cargo:rerun-if-changed=src/hello.c");
}
