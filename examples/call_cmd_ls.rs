// FROM HERE
// https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/std/process/struct.ExitStatus.html

use std::process::Command;

fn main() {
    let status = Command::new("ls")
        .arg("-l")
        .arg(".")
        .status()
        .expect("failed to execute mkdir");

    match status.code() {
        Some(code) => println!("Exited with status code: {}", code),
        None => println!("Process terminated by signal"),
    }
}
