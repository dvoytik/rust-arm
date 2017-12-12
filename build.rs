use std::env;

fn main() {
    let target = env::var("TARGET").
        expect("TARGET env var is not defined");

    if !target.starts_with("arm") {
        panic!("Only ARM architecture is supported. \
               Please consult README.md for building instructions.")
    }

    println!("cargo:rerun-if-changed=build.rs");
}
