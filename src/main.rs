#![windows_subsystem = "windows"]

use std::{
    fs::{self},
    process::Command,
    thread::sleep,
    time::Duration,
};

use seelauncherplus_lib::{
    SEEMTA_BASE_FOLDER, SEEMTA_LAUNCHER_URL, SeeResponse, WEB_CLIENT, get_file_hash,
    get_online_aszf_date, get_seemta_install_dir, launch_see_launcher,
};
use sysinfo::System;

use crate::checker::check_lithium;

mod checker;

const GAME_EXECUTABLE: &str = "gta_sa.exe";
const LAUNCHER_EXECUTABLE: &str = "seemta.exe";

fn is_running() -> bool {
    let mut sys = System::new_all();

    // We need to refresh the process list to ensure we have the latest data
    sys.refresh_all();

    // 2. Iterate through all running processes.
    // processes() returns a reference to a HashMap of processes.
    for (_pid, process) in sys.processes() {
        // 3. Compare the process name (case-insensitive search is safer).
        if process.name().eq_ignore_ascii_case(GAME_EXECUTABLE) {
            println!("[GAME] ✅ Játék megtaláltva (PID: {}).", process.pid());
            return true;
        }
        if process.name().eq_ignore_ascii_case(LAUNCHER_EXECUTABLE) {
            println!(
                "[LAUNCHER] ✅ Launcher megtaláltva (PID: {}).",
                process.pid()
            );
            return true;
        }
    }
    return false;
}

fn check_for_game_or_launcher_running() {
    let mut sys = System::new_all();

    // We need to refresh the process list to ensure we have the latest data
    sys.refresh_all();

    let mut running = false;

    // 2. Iterate through all running processes.
    // processes() returns a reference to a HashMap of processes.
    for (_pid, process) in sys.processes() {
        // 3. Compare the process name (case-insensitive search is safer).
        if process.name().eq_ignore_ascii_case(GAME_EXECUTABLE) {
            println!("[GAME] ✅ Játék megtaláltva (PID: {}).", process.pid());
            running = true;
        }
        if process.name().eq_ignore_ascii_case(LAUNCHER_EXECUTABLE) {
            println!(
                "[LAUNCHER] ✅ Launcher megtaláltva (PID: {}).",
                process.pid()
            );
            running = true;
        }
    }

    if running {
        sleep(Duration::from_secs(30));
        check_for_game_or_launcher_running();
    }
    return;
}

fn runner() {
    if is_running() {
        return;
    }
    let seemta_location = get_seemta_install_dir();
    println!("ÁSZF ellenőrzése...\n");
    let aszf_date = fs::read_to_string(format!("{}\\Terms.see", SEEMTA_BASE_FOLDER));
    if aszf_date.is_err() {
        println!("ÁSZF sikertelen lekérdezése, SeeMTA Launcher indítása...\n");
        launch_see_launcher();
        sleep(Duration::from_secs(10));
        return;
    }
    let online_aszf_date = get_online_aszf_date();
    if online_aszf_date.is_none() {
        println!("Online ÁSZF sikertelen lekérdezése, SeeMTA Launcher indítása...\n");
        launch_see_launcher();
        sleep(Duration::from_secs(10));
        return;
    }

    if aszf_date.unwrap() != online_aszf_date.unwrap() {
        println!("Kérlek fogadd el az új ÁSZF-et a SeeMTA Launcherben!");
        launch_see_launcher();
        sleep(Duration::from_secs(10));
        return;
    }

    let lit = check_lithium();
    if lit {
        launch_see_launcher();
        sleep(Duration::from_secs(10));
        return;
    }

    let check_files = check_files(&seemta_location);
    if check_files {
        launch_see_launcher();
        sleep(Duration::from_secs(10));
        return;
    }

    Command::new("powershell.exe")
            .arg(&format!("Start-Process -FilePath '{}\\Multi Theft Auto.exe' -ArgumentList 'seelaunchernew' -Verb runAs",seemta_location))
            .spawn()
            .unwrap();
    sleep(Duration::from_secs(15));
}

fn main() {
    runner();
    check_for_game_or_launcher_running();
}

fn check_files(game_loc: &String) -> bool {
    let files_req = WEB_CLIENT
        .get(&format!(
            "{}/new/files.php?folder=client",
            SEEMTA_LAUNCHER_URL
        ))
        .send();
    if files_req.is_err() {
        return true;
    }

    let res = files_req.unwrap().json::<SeeResponse>();

    if res.is_err() {
        return true;
    }

    let res = res.unwrap();

    for f in res.1.iter() {
        let file_loc = f.0.replace("\\/", "/");
        println!("{}/{}", game_loc, file_loc);
        let file_hash = get_file_hash(&format!("{}/{}", game_loc, file_loc));
        if file_hash.is_err() {
            return true;
        }
        let file_hash = file_hash.unwrap();
        if f.1 != file_hash {
            println!("{} is not the same", file_loc);
            return true;
        }
    }
    return false;
}
