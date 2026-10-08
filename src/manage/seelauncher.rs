use std::{
    fs::{self, File},
    io,
    process::Command,
};

use reqwest::{blocking::Client, redirect::Policy};

use crate::{SEEMTA_LAUNCHER_URL, WEB_CLIENT, manage::folders::get_app_dir};

pub fn launch_see_launcher() {
    let dir = get_app_dir();
    Command::new("powershell.exe")
        .arg(format!("Start-Process -FilePath '{}/launcher.exe'", dir))
        .spawn()
        .unwrap();
}

pub fn setup_see_launcher() {
    let dir = get_app_dir();
    let launcherexists = fs::exists(format!("{}/launcher.exe", dir)).unwrap();
    if !launcherexists {
        download_see_launcher(&format!("{}/launcher.exe", dir));
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

pub fn get_online_aszf_date() -> Option<String> {
    let req = WEB_CLIENT
        .get("https://rules.see-mta.com/api/ver/aszf")
        .send();
    if req.is_err() {
        return None;
    }
    let text = req.unwrap().text();
    if text.is_err() {
        return None;
    }
    return Some(text.unwrap());
}
