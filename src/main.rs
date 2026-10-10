#![windows_subsystem = "windows"]

use std::{
    fs::{self},
    process::Command,
};

use seelauncherplus_lib::{
    SEEMTA_BASE_FOLDER, get_online_aszf_date, get_seemta_install_dir, launch_see_launcher,
};

use crate::checker::check_lithium;

mod checker;

fn main() {
    let seemta_location = get_seemta_install_dir();
    println!("ÁSZF ellenőrzése...\n");
    let aszf_date = fs::read_to_string(format!("{}\\Terms.see", SEEMTA_BASE_FOLDER));
    if aszf_date.is_err() {
        println!("ÁSZF sikertelen lekérdezése, SeeMTA Launcher indítása...\n");
        launch_see_launcher();
        return;
    }
    let online_aszf_date = get_online_aszf_date();
    if online_aszf_date.is_none() {
        println!("Online ÁSZF sikertelen lekérdezése, SeeMTA Launcher indítása...\n");
        launch_see_launcher();
        return;
    }

    if aszf_date.unwrap() != online_aszf_date.unwrap() {
        println!("Kérlek fogadd el az új ÁSZF-et a SeeMTA Launcherben!");
        launch_see_launcher();
        return;
    }

    let lit = check_lithium();
    if lit {
        launch_see_launcher();
        return;
    }

    Command::new("powershell.exe")
            .arg(&format!("Start-Process -FilePath '{}\\Multi Theft Auto.exe' -ArgumentList 'seelaunchernew' -Verb runAs",seemta_location))
            .spawn()
            .unwrap();
}
