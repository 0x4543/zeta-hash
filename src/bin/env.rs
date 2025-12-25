fn main() {
    let rustc = std::process::Command::new("rustc")
        .arg("--version")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        .unwrap_or_else(|_| "unknown".into());

    println!("crypto-hash version: {}", env!("CARGO_PKG_VERSION"));
    println!("Rust compiler: {}", rustc.trim());
    println!(
        "Platform: {} {}",
        std::env::consts::OS,
        std::env::consts::ARCH
    );
}
