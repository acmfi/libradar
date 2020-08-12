use std::process::Command;

#[test]
fn dex2call_is_installed() {
    let process = Command::new("dex2call")
        .args(&["--help"])
        .output()
        .expect("Failed to execute");
    let out = String::from_utf8(process.stdout).expect("Failed to read");
    println!("{}", out);
    assert_ne!(out, "");
}

#[test]
fn test_dex2call_emulation() {
    let process = Command::new("dex2call")
        .args(&["--all-methods", "resources/tests01.apk"])
        .output()
        .expect("Failed to execute");
    let out = String::from_utf8(process.stdout).expect("Failed to read");
    let mut lines: Vec<&str> = out.split('\n').collect();
    lines.sort();
    for line in lines {
        println!("{}", line);
    }
}
