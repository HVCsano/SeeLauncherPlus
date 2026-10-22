use std::{
    error::Error,
    fs::{self, File},
    io,
};

use reqwest::{blocking::Client, redirect::Policy};
use seelauncherplus_lib::{
    SEEMTA_LAUNCHER_URL, SeeResponse, WEB_CLIENT, get_app_dir, get_file_hash, wait_pause,
};

pub fn setup_see_launcher() -> Result<(), Box<dyn Error>> {
    let dir = get_app_dir();
    let launcher_loc = format!("{}/launcher.exe", dir);
    let launcherexists = fs::exists(&launcher_loc)?;
    if !launcherexists {
        download_see_launcher(&launcher_loc)?;
        return Ok(());
    }
    let launcher_checksum = WEB_CLIENT
        .get(SEEMTA_LAUNCHER_URL.to_string() + "/new/files.php?folder=launcher")
        .send()?;

    let checksums: SeeResponse = launcher_checksum.json().map_err(|e| {
        log::error!("Error on decoding web launcher hash: {:?}", e);
        e
    })?;
    if checksums.0 != "ok" {
        println!("Érvénytelen válasz, valami nem stimmel.");
        wait_pause();
        return Ok(());
    }
    let hash = get_file_hash(&launcher_loc)?;
    if checksums.1[0].1 != hash {
        println!("Frissítés érhető el a launcherhez, letöltés...");
        log::warn!("New update for SeeMTA Launcher, downloading.");
        download_see_launcher(&launcher_loc)?;
    }
    log::info!("SeeMTA Launcher is latest.");
    Ok(())
}

pub fn download_see_launcher(pat: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::builder().redirect(Policy::limited(5)).build()?;

    log::info!("Downloading SeeMTA Launcher.");

    let mut response = client.get(SEEMTA_LAUNCHER_URL).send()?;

    if !response.status().is_success() {
        log::error!("Server returned status: {}", response.status());
    }

    let mut dest_file = File::create(pat)?;

    io::copy(&mut response, &mut dest_file)?;
    log::info!("Finished downloading SeeMTA Launcher.");
    Ok(())
}
