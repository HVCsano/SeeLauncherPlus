#![windows_subsystem = "windows"]

use std::{env, ffi::OsString, process::Command};

fn main() {
    let args: Vec<OsString> = env::args_os().collect();

    if args.len() == 1 {
        Command::new("cmd")
            .args(&["/C", "start", "", "seelauncherplus.exe"])
            .spawn()
            .unwrap();
    }

    if args[1] != "-run" {
        return;
    }

    // futtatás
}
