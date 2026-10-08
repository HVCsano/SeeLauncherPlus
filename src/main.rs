#![windows_subsystem = "windows"]

use std::{
    env,
    ffi::OsString,
    fs::{self},
    process::Command,
};

fn main() {
    let args: Vec<OsString> = env::args_os().collect();

    let mut seemta_location = String::from("C:\\ProgramData\\SeeMTA\\install");

    let seefolder = fs::read_to_string("C:\\ProgramData\\SeeMTA\\NewInstallFolder.see");

    if seefolder.is_ok() {
        seemta_location = seefolder.unwrap();
    }

    if args.len() < 2 {
        return;
    }

    if args[1] == "-run" {
        Command::new("powershell.exe")
            .arg(&format!("Start-Process -FilePath '{}\\Multi Theft Auto.exe' -ArgumentList 'seelaunchernew' -Verb runAs",seemta_location))
            .spawn()
            .unwrap();
    }
}
