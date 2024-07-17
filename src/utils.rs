use std::process::Command;

pub fn get_java_versions() -> Vec<String> {
    let java_folder = "C:\\Program Files\\Java";
    let output = Command::new("cmd")
        .args(&["/C", "dir", java_folder])
        .output()
        .expect("failed to execute process");
    let output = String::from_utf8_lossy(&output.stdout);
    let mut java_versions = Vec::new();
    for line in output.lines() {
        if line.contains("jdk") {
            let version = line.split_whitespace().last().unwrap();
            java_versions.push(version.to_string());
        }
        if line.contains("jre") {
            let version = line.split_whitespace().last().unwrap();
            java_versions.push(version.to_string());
        }
    }
    java_versions
}

pub fn run_jar_with_java(jar_path: &str, java_version: &str, debug: bool) {
    let java_bin = format!("C:\\Program Files\\Java\\{}\\bin\\javaw", java_version);
    let debug_bin = format!("C:\\Program Files\\Java\\{}\\bin\\java", java_version);

    let java_command = if debug { &debug_bin } else { &java_bin };

    println!("Java Bin: {}", java_command);

    let output = Command::new(java_command)
        .arg("-jar")
        .arg(jar_path)
        .output()
        .expect("failed to execute process");

    if output.status.success() {
        println!("Output: {}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }
}
