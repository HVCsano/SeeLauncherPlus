use std::{
    env,
    fs::{self},
    sync::LazyLock,
};

use reqwest::blocking::Client;
use text_io::read;

use crate::manage::seelauncher::launch_see_launcher;

mod manage;

pub const SEEMTA_BASE_FOLDER: &'static str = "C:\\ProgramData\\SeeMTA";
pub const SEEMTA_LAUNCHER_URL: &'static str = "http://client.seega.me";
pub const WEB_CLIENT: LazyLock<Client> = LazyLock::new(reqwest::blocking::Client::new);

fn main() {
    println!("========= SeeLauncher+ =========");
    println!("= Készítette: Csanó (csano.hu) =");
    println!("================================\n");

    println!("========= Előkészítés ==========\n");
    manage::folders::setup_app_dir();

    println!("SeeMTA Launcher ellenőrzése...\n");
    manage::seelauncher::setup_see_launcher();

    println!("ÁSZF ellenőrzése...\n");
    let aszf_date = fs::read_to_string(format!("{}\\Terms.see", SEEMTA_BASE_FOLDER));
    if aszf_date.is_err() {
        println!("ÁSZF sikertelen lekérdezése, SeeMTA Launcher indítása...\n");
        launch_see_launcher();
    }
    let online_aszf_date = manage::seelauncher::get_online_aszf_date();
    if online_aszf_date.is_none() {
        println!("Online ÁSZF sikertelen lekérdezése, SeeMTA Launcher indítása...\n");
        launch_see_launcher();
    }

    if aszf_date.unwrap() != online_aszf_date.unwrap() {
        println!("Kérlek fogadd el az új ÁSZF-et a SeeMTA Launcherben!");
        launch_see_launcher();
    }

    println!("Előkészítés sikeres, üdv!");
    send_menu();
}

fn send_menu() {
    println!("\n======== Választó menü =========\n");
    println!("Kérlek írd be a következő számok valamelyikék a menüpontokhoz!");
    println!(
        "[1] SeeMTA indítása\n[2] Steam óraszámlálás beállítása\n[3] SeeLauncher+ automatikus játékindítás beállítása\n[4] Asztali vagy Start-menü parancsikon létrehozása a SeeLauncher+ számára\n[5] Alkalmazásból kilépés\nSeeLauncher+ Verzió: v{}\n",
        env!("CARGO_PKG_VERSION")
    );

    let choice: i8 = read!();

    match choice {
        1 => {
            println!("\nSeeMTA indítása...");
            manage::launch_seemta();
        }
        5 => {
            println!("\nJó volt veled, szia!");
            return;
        }
        _ => {
            println!("\nÉrvénytelen választás, próbáld újra!");
            send_menu();
        }
    }
}
