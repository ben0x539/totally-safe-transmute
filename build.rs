use std::process;

fn main() {
    if !process::Command::new("cargo")
        .args(&["build", "--release"])
        .current_dir("helper")
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .success() { panic!("oh no"); }
}