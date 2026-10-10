use std::{fs, io, path::Path};

use rfd::FileDialog;
use seelauncherplus_lib::{
    SEEMTA_BASE_FOLDER, get_seemta_install_dir, launch_see_launcher, wait_pause,
};
use text_io::read;

pub fn change_seemta_folder() {
    let current = get_seemta_install_dir();
    println!("Aktuális mappa: {}\nÁthelyezzük? [igen/nem]", current);
    let res: String = read!();
    if res != "igen".to_string() {
        return;
    }
    println!("A mappaválasztó megnyílik, kérlek ott válaszd ki a mappát!");
    let folder = FileDialog::new()
        .set_title("Válaszd ki az új SeeMTA mappát!")
        .set_directory(&current)
        .pick_folder();
    match folder {
        Some(newdir) => {
            let mut errorhappened = false;
            let newdirstr = newdir.to_str().unwrap();
            println!("Az új mappa: {}", newdirstr);

            println!("Config mappa áthelyezése...");
            if let Err(err) = copy_files("config", Path::new(&current), &newdir) {
                errorhappened = true;
                println!("A config mappa másolása sikertelen. ({:?})", err);
            }
            println!("Mods mappa áthelyezése...");
            if let Err(err) = copy_files("mods", Path::new(&current), &newdir) {
                errorhappened = true;
                println!("A mods mappa másolása sikertelen. ({:?})", err);
            }
            println!("Redist mappa áthelyezése...");
            if let Err(err) = copy_files("redist", Path::new(&current), &newdir) {
                errorhappened = true;
                println!("A redist mappa másolása sikertelen. ({:?})", err);
            }
            println!("Screenshots mappa áthelyezése...");
            if let Err(err) = copy_files("screenshots", Path::new(&current), &newdir) {
                errorhappened = true;
                println!("A screenshots mappa másolása sikertelen. ({:?})", err);
            }
            println!("Skins mappa áthelyezése...");
            if let Err(err) = copy_files("skins", Path::new(&current), &newdir) {
                errorhappened = true;
                println!("A skins mappa másolása sikertelen. ({:?})", err);
            }
            println!("Alapfájlok áthelyezése...");
            if let Err(err) = copy_files("Multi Theft Auto.exe", Path::new(&current), &newdir) {
                errorhappened = true;
                println!(
                    "A Multi Teft Auto.exe fájl átmásolása sikertelen. ({:?})",
                    err
                );
            }
            if let Err(err) = copy_files("ver.see", Path::new(&current), &newdir) {
                errorhappened = true;
                println!("A ver.see fájl másolása sikertelen. ({:?})", err);
            }
            if !errorhappened {
                let res = fs::write(
                    format!("{}/NewInstallFolder.see", SEEMTA_BASE_FOLDER),
                    newdirstr,
                );

                if res.is_err() {
                    println!(
                        "NewInstallFolder.see felülírása sikertelen. ({:?})",
                        res.unwrap_err()
                    );
                }

                println!(
                    "\nMinden sikeresen zajlott! A régi mappából a fájlokat nem töröljük, azt rád bízzuk!\nA biztonság kedvéért elindítjuk a launchert, hogy az minden esetleges hiányt rendbehozzon, a játékot ott elindítani nem kell!"
                );
                wait_pause();
                launch_see_launcher();
                return;
            }

            if errorhappened {
                println!(
                    "Valami hiba történt, kérlek nézd vissza hol a baj, amennyiben valami nem stimmel, keress fel fórumon!\nA teljes változtatás nem ment végbe, a régi mappában használható a SeeMTA!"
                )
            }
        }
        None => {
            println!("Nem került új mappa kiválasztásra.");
            return;
        }
    }
}

fn copy_files(name: &str, old_dir: &Path, new_dir: &Path) -> io::Result<()> {
    copy_path(&old_dir.join(name), &new_dir.join(name))
}

fn copy_path(source: &Path, destination: &Path) -> io::Result<()> {
    let metadata = fs::metadata(source)?;
    if metadata.is_dir() {
        fs::create_dir_all(destination)?;
        for entry in fs::read_dir(source)? {
            let entry = entry?;
            copy_path(&entry.path(), &destination.join(entry.file_name()))?;
        }
    } else {
        fs::copy(source, destination)?;
    }
    Ok(())
}
