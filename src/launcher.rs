use std::env;

use seelauncherplus_lib::{launch_see_launcher, wait_pause};
use text_io::read;

use crate::manage::shortcut::get_seelauncherplus_loc;
mod manage;

fn main() {
    println!("========= SeeLauncher+ =========");
    println!("= Készítette: Csanó (csano.hu) =");
    println!("================================\n");

    println!("========= Előkészítés ==========\n");
    manage::folders::setup_app_dir();

    println!("SeeMTA Launcher ellenőrzése...\n");
    manage::seelauncher::setup_see_launcher();

    println!("Előkészítés sikeres, üdv!");
    send_menu();
}

fn send_menu() {
    println!("\n======== Választó menü =========\n");
    println!("Kérlek írd be a következő számok valamelyikék a menüpontokhoz!");
    println!(
        "[1] SeeMTA indítása\n[2] SeeMTA Launcher indítása\n[3] SeeMTA telepítési mappa áthelyezése BÁRHOVA\n[4] Steam óraszámlálás beállítása\n[5] SeeLauncher+ automatikus játékindítás beállítása\n[6] Asztali vagy Start-menü parancsikon létrehozása a SeeLauncher+ számára\n[7] Alkalmazásból kilépés\nSeeLauncher+ Verzió: v{}\n",
        env!("CARGO_PKG_VERSION")
    );

    let choice: i8 = read!();

    match choice {
        1 => {
            println!("\nSeeMTA indítása...");
            manage::launch_seemta();
        }
        2 => {
            println!("\nSeeMTA Launcher indítása...");
            launch_see_launcher();
        }
        3 => {
            manage::change_folder::change_seemta_folder();
            send_menu();
        }
        4 => {
            println!("\nSteam óraszámlálás");
            let launcher_loc = get_seelauncherplus_loc();
            if launcher_loc.is_none() {
                println!("Fent a bibi. :(");
                send_menu();
            }
            if launcher_loc.is_some() {
                println!(
                    "Ehhez nincs más dolgod, mint amennyiben a GTA:SA Steamről van meg, és letöltve, beírni a Tulajdonságok->Indítási opciók-hoz, hogy:"
                );
                println!("'{}' %command%", launcher_loc.unwrap());
                wait_pause();
                send_menu();
            }
        }
        5 => {
            println!(
                "\nAz automata játékindításhoz nincs más dolgod, mint a seelauncherplus.exe fájlt elindítani, a seelauncherplus-launcher.exe helyett!"
            );
            wait_pause();

            send_menu();
        }
        6 => {
            println!("\nA parancsikonok létrejönnek bármely gomb lenyomásakor!");
            wait_pause();
            manage::shortcut::create_shortcuts();
            println!("\nA parancsikonok létrehozva!");
            wait_pause();
            send_menu();
        }
        7 => {
            println!("\nJó volt veled, szia!");
            return;
        }
        _ => {
            println!("\nÉrvénytelen választás, próbáld újra!");
            send_menu();
        }
    }
}
