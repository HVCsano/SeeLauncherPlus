use std::{
    error::Error,
    fs::{self, create_dir, remove_file},
    process::Command,
};

use seelauncherplus_lib::get_app_dir;

pub fn open_in_explorer(dir: &String) -> Result<(), Box<dyn Error>> {
    Command::new("explorer")
        .arg(dir.replace("/", "\\"))
        .spawn()?;
    Ok(())
}

pub fn setup_app_dir() {
    let pat = get_app_dir();

    let folder = fs::exists(&pat);
    if folder.is_err() || !folder.unwrap() {
        create_dir(&pat).unwrap();
        create_dir(format!("{}/logs", &pat)).unwrap();
        create_dir(format!("{}/seefiles", &pat)).unwrap();
        return;
    }
    let meta = fs::metadata(&pat).unwrap();

    if !meta.is_dir() {
        remove_file(&pat).unwrap();
        create_dir(&pat).unwrap();
        create_dir(format!("{}/logs", &pat)).unwrap();
        create_dir(format!("{}/seefiles", &pat)).unwrap();
        return;
    }

    let folder = fs::exists(&format!("{}/logs", &pat));
    if folder.is_err() || !folder.unwrap() {
        create_dir(&format!("{}/logs", &pat)).unwrap();
    }
    let meta = fs::metadata(&format!("{}/logs", &pat)).unwrap();

    if !meta.is_dir() {
        remove_file(&pat).unwrap();
        create_dir(format!("{}/logs", &pat)).unwrap();
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
