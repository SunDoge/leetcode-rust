use std::process;

fn main() {
    // 这个脚本运行结果会被cache，不是每次build都会rerun。
    // rerun-if-changed没法recursive监听src的变化。
    // 反正偶尔能触发。
    let status = process::Command::new("python")
        .arg("gen_readme.py")
        .status()
        .expect("Fail to generage README.md");

    assert!(status.success());
}
