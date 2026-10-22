use std::{env, error::Error, path::Path, thread::sleep, time::Duration};

use ftail::Ftail;
use log::LevelFilter;
use seelauncherplus_lib::{
    get_app_dir, launch_see_launcher,
    update_check::{get_latest_tag, is_latest},
    wait_pause,
};
use text_io::read;

use crate::manage::{folders::open_in_explorer, shortcut::get_seelauncherplus_loc};
mod manage;

fn main() -> Result<(), Box<dyn Error>> {
    println!("========= SeeLauncher+ =========");
    println!("= Készítette: Csanó (csano.hu) =");
    println!("================================\n");
    println!("Telepített verzió: v{}", env!("CARGO_PKG_VERSION"));

    println!("\n========= Előkészítés ==========\n");
    manage::folders::setup_app_dir();

    let app_dir = get_app_dir();

    Ftail::new()
        .daily_file(&Path::new(&format!("{}/logs", app_dir)), LevelFilter::Debug)
        .init()?;

    let latest = get_latest_tag();

    if latest == "".to_string() {
        log::error!("Failed to get latest version!");
        println!(
            "Legújabb verzió lekérése sikertelen, ha ez sokáig fennáll, kérlek ellenőrizd manuálisan!\nLegújabb verzió: https://github.com/HVCsano/SeeLauncherPlus/releases/latest"
        )
    }

    if !is_latest(&latest) {
        log::warn!("Not using latest version!");
        println!("\nNem a legújabb verziót használod!");
        println!(
            "A teljes felhasználói élményért kérlek töltsd le a legújabb verziót innen: https://github.com/HVCsano/SeeLauncherPlus/releases/latest.\n"
        )
    }

    log::info!("Starting version: {}.", env!("CARGO_PKG_VERSION"));
    log::info!("Latest version: {}.", latest);
    println!("SeeMTA Launcher ellenőrzése...\n");
    manage::seelauncher::setup_see_launcher()?;

    println!("Előkészítés sikeres, üdv!");
    send_menu()?;
    Ok(())
}

fn send_menu() -> Result<(), Box<dyn Error>> {
    println!("\n======== Választó menü =========\n");
    println!("Kérlek írd be a következő számok valamelyikék a menüpontokhoz!");
    println!(
        "[1] SeeMTA indítása\n[2] SeeMTA Launcher indítása\n[3] SeeMTA telepítési mappa áthelyezése BÁRHOVA\n[4] Steam óraszámlálás beállítása\n[5] SeeLauncher+ automatikus játékindítás beállítása\n[6] Asztali vagy Start-menü parancsikon létrehozása a SeeLauncher+ számára\n[7] SeeLauncher+ log mappa megnyitása\n[8] Alkalmazásból kilépés\n",
    );

    let choice: i8 = read!();

    match choice {
        1 => {
            println!("\nSeeMTA indítása...");
            manage::launch_seemta()
        }
        2 => {
            println!("\nSeeMTA Launcher indítása...");
            launch_see_launcher()
        }
        3 => {
            manage::change_folder::change_seemta_folder()?;
            send_menu()
        }
        4 => {
            println!("\nSteam óraszámlálás");
            let launcher_loc = get_seelauncherplus_loc();
            if launcher_loc.is_none() {
                println!("Fent a bibi. :(");
                return send_menu();
            }
            if launcher_loc.is_some() {
                println!(
                    "Ehhez nincs más dolgod, mint amennyiben a GTA:SA Steamről van meg, és letöltve, beírni a Tulajdonságok->Indítási opciók-hoz, hogy:"
                );
                println!("\"{}\" %command%", launcher_loc.unwrap());
                wait_pause();
                return send_menu();
            }
            Ok(())
        }
        5 => {
            println!(
                "\nAz automata játékindításhoz nincs más dolgod, mint a seelauncherplus.exe fájlt elindítani, a seelauncherplus-launcher.exe helyett!"
            );
            wait_pause();

            send_menu()
        }
        6 => {
            println!("\nA parancsikonok létrejönnek bármely gomb lenyomásakor!");
            wait_pause();
            manage::shortcut::create_shortcuts();
            println!("\nA parancsikonok létrehozva!");
            wait_pause();
            send_menu()
        }
        7 => {
            println!("\nMappa megnyitása...");
            open_in_explorer(&format!("{}/logs", get_app_dir()))?;
            send_menu()
        }
        8 => {
            println!("\nJó volt veled, szia!");
            sleep(Duration::from_millis(500));
            return Ok(());
        }
        _ => {
            println!("\nÉrvénytelen választás, próbáld újra!");
            send_menu()
        }
    }
}
