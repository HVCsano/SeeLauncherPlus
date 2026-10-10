use std::{
    fs::{self, File},
    io,
};

use reqwest::{blocking::Client, redirect::Policy};
use seelauncherplus_lib::{WEB_CLIENT, get_app_dir, get_file_hash, wait_pause};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Response(String, Vec<(String, String, String)>);

use crate::SEEMTA_LAUNCHER_URL;

pub fn setup_see_launcher() {
    let dir = get_app_dir();
    let launcher_loc = format!("{}/launcher.exe", dir);
    let launcherexists = fs::exists(&launcher_loc).unwrap();
    if !launcherexists {
        download_see_launcher(&launcher_loc);
        return;
    }
    let launcher_checksum = WEB_CLIENT
        .get("https://client.seega.me/new/files.php?folder=launcher")
        .send();
    if launcher_checksum.is_err() {
        println!("Launcher checksum lekérése sikertelen, van interneted?");
        wait_pause();
        return;
    }

    let checksums: Result<Response, reqwest::Error> = launcher_checksum.unwrap().json();
    if checksums.is_err() {
        println!("Érvénytelen válasz, valami nem stimmel.");
        wait_pause();
        return;
    }
    let checksums = checksums.unwrap();
    if checksums.0 != "ok" {
        println!("Érvénytelen válasz, valami nem stimmel.");
        wait_pause();
        return;
    }
    let hash = get_file_hash(&launcher_loc);
    if hash.is_err() {
        println!("Launcher hash lekérése sikertelen, keress fel fórumon!");
        wait_pause();
        return;
    }
    let hash = hash.unwrap();
    if checksums.1[0].1 != hash {
        println!("Frissítés érhető el a launcherhez, letöltés...");
        download_see_launcher(&launcher_loc);
    }
}

pub fn download_see_launcher(pat: &str) {
    let client = Client::builder()
        .redirect(Policy::limited(5))
        .build()
        .unwrap();

    let mut response = client.get(SEEMTA_LAUNCHER_URL).send().unwrap();

    if !response.status().is_success() {
        println!("Server returned status: {}", response.status());
    }

    let mut dest_file = File::create(pat).unwrap();

    io::copy(&mut response, &mut dest_file).unwrap();
}
