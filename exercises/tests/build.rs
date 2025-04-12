//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.
use std::time::SystemTime;

fn main() {
    // 设置环境变量 TEST_FOO，值为当前的时间戳
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!("cargo:rerun-if-env-changed=TEST_FOO"); // 当 TEST_FOO 环境变量改变时，重新运行 build.rs
    println!("cargo:TEST_FOO={}", timestamp); // 设置 TEST_FOO 为当前时间戳

    // 启用 `pass` 特性，在 `tests8.rs` 中返回时测试通过
    let enable_pass_feature = "cargo:rustc-cfg=feature=\"pass\""; // 启用 'pass' 特性
    println!("{}", enable_pass_feature); // 启用 'pass' 特性
}