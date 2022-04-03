use std::env;

fn main() {
    println!("cargo:rustc-link-lib=bela");
    println!("cargo:rustc-link-lib=belaextra");
    println!("cargo:rustc-link-lib=seasocks");
    println!("cargo:rustc-link-lib=NE10");
    println!("cargo:rustc-link-lib=asound");
    println!("cargo:rustc-link-lib=cobalt");
    println!("cargo:rustc-link-lib=modechk");
    println!("cargo:rustc-link-lib=pthread");
    println!("cargo:rustc-link-lib=rt");
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-lib=prussdrv");
    println!(
        "cargo:rustc-link-search=all={}/lib",
        env::var("CARGO_MANIFEST_DIR").unwrap()
    );
}
