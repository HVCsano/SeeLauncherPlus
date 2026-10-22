#![windows_subsystem = "windows"]

use std::{
    error::Error,
    fs::{self},
    path::Path,
    process::Command,
    thread::sleep,
    time::Duration,
};

use ftail::Ftail;
use log::LevelFilter;
use seelauncherplus_lib::{
    SEEMTA_BASE_FOLDER, SEEMTA_LAUNCHER_URL, SeeResponse, WEB_CLIENT, get_app_dir, get_file_hash,
    get_online_aszf_date, get_seemta_install_dir, launch_see_launcher,
    update_check::{get_latest_tag, is_latest},
};
use sysinfo::System;
use win_msgbox::YesNo;

use crate::checker::check_lithium;

mod checker;

const GAME_EXECUTABLE: &str = "gta_sa.exe";
const LAUNCHER_EXECUTABLE: &str = "seemta.exe";

fn is_running() -> bool {
    let mut sys = System::new();

    sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);

    for (_pid, process) in sys.processes() {
        if process.name().eq_ignore_ascii_case(GAME_EXECUTABLE) {
            log::info!("[GAME] ✅ Játék megtaláltva (PID: {}).", process.pid());
            return true;
        }
        if process.name().eq_ignore_ascii_case(LAUNCHER_EXECUTABLE) {
            log::info!(
                "[LAUNCHER] ✅ Launcher megtaláltva (PID: {}).",
                process.pid()
            );
            return true;
        }
    }

    return false;
}

fn check_for_game_or_launcher_running() {
    let mut sys = System::new();

    loop {
        sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);

        let mut running = false;

        for (_pid, process) in sys.processes() {
            if process.name().eq_ignore_ascii_case(GAME_EXECUTABLE) {
                log::info!("[GAME] ✅ Játék megtaláltva (PID: {}).", process.pid());
                running = true;
            }
            if process.name().eq_ignore_ascii_case(LAUNCHER_EXECUTABLE) {
                log::info!(
                    "[LAUNCHER] ✅ Launcher megtaláltva (PID: {}).",
                    process.pid()
                );
                running = true;
            }
        }

        if !running {
            break;
        }
        sleep(Duration::from_secs(30));
    }
}

fn runner() -> Result<(), Box<dyn Error>> {
    if is_running() {
        return Ok(());
    }
    let seemta_location = get_seemta_install_dir();
    println!("ÁSZF ellenőrzése...\n");
    let aszf_date = fs::read_to_string(format!("{}\\Terms.see", SEEMTA_BASE_FOLDER));
    if aszf_date.is_err() {
        println!("ÁSZF sikertelen lekérdezése, SeeMTA Launcher indítása...\n");
        launch_see_launcher()?;
        sleep(Duration::from_secs(10));
        return Ok(());
    }
    let online_aszf_date = get_online_aszf_date();
    if online_aszf_date.is_none() {
        println!("Online ÁSZF sikertelen lekérdezése, SeeMTA Launcher indítása...\n");
        launch_see_launcher()?;
        sleep(Duration::from_secs(10));
        return Ok(());
    }

    if aszf_date.unwrap() != online_aszf_date.unwrap() {
        println!("Kérlek fogadd el az új ÁSZF-et a SeeMTA Launcherben!");
        launch_see_launcher()?;
        sleep(Duration::from_secs(10));
        return Ok(());
    }

    let lit = check_lithium();
    if lit.is_err() {
        launch_see_launcher()?;
        sleep(Duration::from_secs(10));
        return Ok(());
    }

    let check_files = check_files(&seemta_location);
    if check_files {
        launch_see_launcher()?;
        sleep(Duration::from_secs(10));
        return Ok(());
    }

    Command::new("powershell.exe")
            .arg(&format!("Start-Process -FilePath '{}\\Multi Theft Auto.exe' -ArgumentList 'seelaunchernew' -Verb runAs",seemta_location))
            .spawn()?;
    sleep(Duration::from_secs(15));
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let app_dir = get_app_dir();

    Ftail::new()
        .daily_file(&Path::new(&format!("{}/logs", app_dir)), LevelFilter::Debug)
        .init()?;

    log::info!("Starting version: {}.", env!("CARGO_PKG_VERSION"));

    if !is_latest(&get_latest_tag()) {
        let selection = win_msgbox::warning::<YesNo>("Egy új frissítés elérhető a SeeLauncher+-hoz! A legújabb javításokhoz, funkciókhoz érdemes frissíteni. Frissítés most?")
            .title("SeeLauncher+ Frissítés elérhető!")
            .show()
            .expect("Failed to show message box");

        if selection == YesNo::Yes {
            let _ =
                open::that_detached("https://github.com/HVCsano/SeeLauncherPlus/releases/latest");
        }
    }

    runner()?;
    check_for_game_or_launcher_running();
    Ok(())
}

fn check_files(game_loc: &String) -> bool {
    let files_req = WEB_CLIENT
        .get(&format!(
            "{}/new/files.php?folder=client",
            SEEMTA_LAUNCHER_URL
        ))
        .send();
    if files_req.is_err() {
        log::error!("Failed checking files: {}.", files_req.unwrap_err());

        return true;
    }

    let res = files_req.unwrap().json::<SeeResponse>();

    if res.is_err() {
        log::error!("Failed parsing client response: {}.", res.unwrap_err());
        return true;
    }

    let res = res.unwrap();

    for f in res.1.iter() {
        let file_loc = f.0.replace("\\/", "/");
        println!("{}/{}", game_loc, file_loc);
        let file_hash = get_file_hash(&format!("{}/{}", game_loc, file_loc));
        if file_hash.is_err() {
            log::error!(
                "Failed getting {} hash: {}.",
                file_loc,
                file_hash.unwrap_err()
            );
            return true;
        }
        let file_hash = file_hash.unwrap();
        if f.1 != file_hash {
            log::error!("{} is not the same hash: {}.", file_loc, file_hash);
            println!("{} is not the same", file_loc);
            return true;
        }
    }
    return false;
}
