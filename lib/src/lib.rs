use std::{
    fs::{self, File},
    io,
    process::Command,
    sync::LazyLock,
};

use reqwest::blocking::Client;
use sha2::{Digest, Sha256};

pub const WEB_CLIENT: LazyLock<Client> = LazyLock::new(reqwest::blocking::Client::new);
pub const SEEMTA_BASE_FOLDER: &'static str = "C:\\ProgramData\\SeeMTA";

pub fn get_app_dir() -> String {
    let base_dir = dirs::data_local_dir().unwrap();

    return format!("{}/seelauncherplus", base_dir.to_str().unwrap());
}

pub fn launch_see_launcher() {
    let dir = get_app_dir();
    Command::new("powershell.exe")
        .arg(format!("Start-Process -FilePath '{}/launcher.exe'", dir))
        .spawn()
        .unwrap();
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

pub fn get_seemta_install_dir() -> String {
    let mut seemta_location = String::from(SEEMTA_BASE_FOLDER.to_string() + "\\install");

    let seefolder = fs::read_to_string(SEEMTA_BASE_FOLDER.to_string() + "\\NewInstallFolder.see");

    if seefolder.is_ok() {
        seemta_location = seefolder.unwrap();
    }

    return seemta_location;
}

pub fn wait_pause() {
    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
}

pub fn get_file_hash(file: &String) -> io::Result<String> {
    let mut file = File::open(&file)?;
    let mut hasher = Sha256::new();

    io::copy(&mut file, &mut hasher)?;

    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}
