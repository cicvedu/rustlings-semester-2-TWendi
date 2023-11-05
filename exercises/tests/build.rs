//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    // 对于 tests7，我们需要设置一个名为 `TEST_FOO` 的环境变量。
    // 使用当前的 Unix 时间戳来设置这个变量。
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // 对于 tests8，我们需要启用 "pass" 特性。
    // 这个命令会告诉 Cargo 启用这个特性。
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
