use std::process;

fn main() {
    println!("cargo:rerun-if-changed=helper/src/main.rs");
    if !process::Command::new("cargo")
        .args(&["build", "--release"])
        .current_dir("helper")
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .success() { panic!("oh no"); }
}