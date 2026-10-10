use std::{env, fs};

use dirs::{desktop_dir, home_dir};
use mslnk::ShellLink;

pub fn get_seelauncherplus_loc() -> Option<String> {
    match env::current_exe() {
        Ok(exe_path) => {
            let folder = exe_path.parent().unwrap();

            let launcher_loc = format!("{}/seelauncherplus.exe", folder.to_str().unwrap());

            if !fs::exists(&launcher_loc).unwrap() {
                println!(
                    "seelauncherplus.exe nem található ugyanabban a mappában, mint a seelauncherplus-launcher.exe, kérlek töltsd le a fájlt!",
                );
                return None;
            }
            return Some(launcher_loc);
        }
        Err(err) => {
            println!("Mappalekérdezés sikertelen: {:?}", err);
            return None;
        }
    }
}

pub fn create_shortcuts() {
    let launcher_loc = get_seelauncherplus_loc();

    if launcher_loc.is_none() {
        println!("Fenti hiba miatt a parancsikonok létrehozása sikertelen.",);
        return;
    }
    let launcher_loc = launcher_loc.unwrap();

    let desktop = desktop_dir().unwrap();

    let sl = ShellLink::new(launcher_loc).unwrap();
    let res = sl.create_lnk(format!("{}/SeeMTA Indítása.lnk", desktop.to_str().unwrap()));
    if res.is_err() {
        println!(
            "Asztali parancsikon létrehozása sikertelen: {:?}",
            res.unwrap_err()
        )
    }

    let home = home_dir().unwrap();

    let res = sl.create_lnk(format!(
        "{}/AppData/Roaming/Microsoft/Windows/Start Menu/Programs/SeeMTA Indítása.lnk",
        home.to_str().unwrap()
    ));
    if res.is_err() {
        println!(
            "Start-menü parancsikon létrehozása sikertelen: {:?}",
            res.unwrap_err()
        )
    }
}
