use std::fs;
use std::process::Command;

fn main() {
    let skip_args = if let Ok(_) = std::env::var("CARGO") {
      2
    } else {
      1
    };
    let cargo_args: Vec<String> = std::env::args().skip(skip_args).collect();

    println!("{:?}", cargo_args);
    let output = Command::new("cargo")
      .arg("new")
      .args(&cargo_args)
      .output()
      .expect("Failed to execute cargo");

    if !output.status.success() {
        println!("cargo failed with:");
        println!("status: {}", output.status);
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        std::process::exit(1);
    } else {
      println!("{}", String::from_utf8_lossy(&output.stdout));
      eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    }

    let fmt = include_str!("config_files/rustfmt.toml");
    // write the string to file rustfmt.toml
    fs::write(format!("{}/rustfmt.toml", cargo_args[0]), fmt).expect("Unable to write rustfmt.toml file");

    let just = include_str!("config_files/justfile");
    fs::write(format!("{}/justfile", cargo_args[0]), just).expect("Unable to write Justfile file");
}

