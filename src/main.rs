#![windows_subsystem = "windows"]
use std::{fs, io, io::prelude::*, process::Command, env::args};

fn main() {
    let path = if args().count() == 1 { "modorganizer2/instance_path.txt" } else {"modorganizer2/instance_download_path.txt" };
    let rn = fs::read_to_string(path)
        .expect("Error reading instance path")
        .trim()
        .to_string();
    let mut cmd = Command::new(rn);
    cmd.args(args().skip(1));
    println!(
        "{:?}",
        cmd.spawn()
            .expect("error spawning process")
            .wait_with_output()
            .expect("error running program")
            .status
    );
    let mut stdin = io::stdin();
    let _ = stdin.read(&mut [0u8]);
}
