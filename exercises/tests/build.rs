//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.
use std::time::SystemTime;

fn main() {
    // 设置环境变量 TEST_FOO，值为当前的时间戳
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!("cargo:rustc-env=TEST_FOO={}", now);

    // 启用 `pass` 特性，在 `tests8.rs` 中返回时测试通过
    let enable_pass_feature = "cargo:rustc-cfg=feature=\"pass\""; // 启用 'pass' 特性
    println!("{}", enable_pass_feature); // 启用 'pass' 特性
}