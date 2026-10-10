use seelauncherplus_lib::{SEEMTA_LAUNCHER_URL, SeeResponse, WEB_CLIENT, get_file_hash};

pub fn check_lithium() -> bool {
    let get = WEB_CLIENT
        .get(SEEMTA_LAUNCHER_URL.to_string() + "/new/files.php?folder=lithium")
        .send();
    if get.is_err() {
        return true;
    }
    let lithium_check: Result<SeeResponse, reqwest::Error> = get.unwrap().json();
    if lithium_check.is_err() {
        return true;
    }
    let hash = get_file_hash(&"C:\\ProgramData\\SeeMTA All\\1.5\\SeeLithium.sys".to_string());
    if hash.is_err() {
        return true;
    }
    if lithium_check.unwrap().1[0].1 != hash.unwrap() {
        return true;
    }
    return false;
}
