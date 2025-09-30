use std::{env, path::Path, process};

fn main() {
    let ui_dir = Path::new("./ui");
    env::set_current_dir(ui_dir).unwrap();

    let ui_build_status = process::Command::new("npm")
        .arg("run")
        .arg("build")
        .status();

    assert!(ui_build_status.is_ok_and(|status| status.success()));
}
