use std::error::Error;

use seelauncherplus_lib::{SEEMTA_LAUNCHER_URL, SeeResponse, WEB_CLIENT, get_file_hash};

pub fn check_lithium() -> Result<(), Box<dyn Error>> {
    let get = WEB_CLIENT
        .get(SEEMTA_LAUNCHER_URL.to_string() + "/new/files.php?folder=lithium")
        .send()?;
    let lithium_check: SeeResponse = get.json()?;
    let hash = get_file_hash(&"C:\\ProgramData\\SeeMTA All\\1.5\\SeeLithium.sys".to_string())?;
    if lithium_check.1[0].1 != hash {
        return Err("Invalid hash".into());
    }
    return Ok(());
}
