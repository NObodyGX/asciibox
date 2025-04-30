use std::{env, fs, path::PathBuf, process::Command};

fn get_name() -> String {
    let root_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let content = fs::read_to_string(root_dir.join("Cargo.toml")).unwrap();
    for line in content.split("\n") {
        if !line.starts_with("name = ") {
            continue;
        }
        let name = line.replace("name =", "").replace("\"", "");
        return name.trim().to_string();
    }
    return "".to_string();
}

fn pack_gresource() {
    let root_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let name = get_name();
    let target = format!("{}.gresource", name);
    let target = &target;
    let xml_target = format!("data/{}.gresource.xml", name);
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    // 编译 gresource
    glib_build_tools::compile_resources(&["data"], &xml_target, target);
    // 将其拷贝到工作目录
    let _ = fs::copy(
        out_dir.join(target),
        root_dir.join("data").join("bin").join(target),
    );
}

// 这里理论上也可以使用 meson setup 生成后然后 copy,
fn update_conf() {
    let root_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    let conf_in = root_dir.join("src").join("conf.rs.in");
    let conf_out = root_dir.join("src").join("conf.rs");
    if conf_out.exists() {
        println!("输出文件已存在，跳过更新 conf.rs.in");
        return;
    }
    if !conf_in.exists() {
        println!("conf.rs.in 文件不存在");
        return;
    }
    let out_dir_name = "__build";
    let out_dir = root_dir.join(out_dir_name);
    // 在项目目录下运行 meson setup
    let output = Command::new("meson")
        .current_dir(root_dir)
        .arg("setup")
        .arg(out_dir_name)
        .output()
        .expect("failed to run meson setup");
    if !output.status.success() {
        panic!(
            "meson setup failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    let _ = fs::copy(out_dir.join("conf.rs"), conf_out);
    // 删除临时文件夹
    let _ = fs::remove_dir_all(out_dir);
}

fn main() {
    pack_gresource();
    update_conf();
}
