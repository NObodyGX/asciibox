use std::{env,path::PathBuf, process::Command};


fn main() {
    let root_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let build_script = root_dir.join("build.sh");
    let output = Command::new("bash")
        .arg(build_script)
        .arg("prepare")
        .output()
        .expect("failed to run build.sh");
    if !output.status.success() {
        panic!(
            "meson setup failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
