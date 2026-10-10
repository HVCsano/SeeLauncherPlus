use std::fs::{self, create_dir, remove_file};

use seelauncherplus_lib::get_app_dir;

pub fn setup_app_dir() {
    let pat = get_app_dir();

    let folder = fs::exists(&pat);
    if folder.is_err() || !folder.unwrap() {
        create_dir(&pat).unwrap();
        create_dir(format!("{}/cache", &pat)).unwrap();
        create_dir(format!("{}/seefiles", &pat)).unwrap();
        return;
    }
    let meta = fs::metadata(&pat).unwrap();

    if !meta.is_dir() {
        remove_file(&pat).unwrap();
        create_dir(&pat).unwrap();
        create_dir(format!("{}/cache", &pat)).unwrap();
        create_dir(format!("{}/seefiles", &pat)).unwrap();
        return;
    }

    let folder = fs::exists(&format!("{}/cache", &pat));
    if folder.is_err() || !folder.unwrap() {
        create_dir(&format!("{}/cache", &pat)).unwrap();
    }
    let meta = fs::metadata(&format!("{}/cache", &pat)).unwrap();

    if !meta.is_dir() {
        remove_file(&pat).unwrap();
        create_dir(format!("{}/cache", &pat)).unwrap();
        return;
    }

    let folder = fs::exists(&format!("{}/seefiles", &pat));
    if folder.is_err() || !folder.unwrap() {
        create_dir(&format!("{}/seefiles", &pat)).unwrap();
    }
    let meta = fs::metadata(&format!("{}/seefiles", &pat)).unwrap();

    if !meta.is_dir() {
        remove_file(&pat).unwrap();
        create_dir(format!("{}/seefiles", &pat)).unwrap();
        return;
    }
}
