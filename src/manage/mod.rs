use std::{error::Error, process::Command};

pub mod change_folder;
pub mod folders;
pub mod seelauncher;
pub mod shortcut;

pub fn launch_seemta() -> Result<(), Box<dyn Error>> {
    Command::new("powershell.exe")
        .arg("Start-Process -FilePath 'seelauncherplus.exe'")
        .spawn()?;
    Ok(())
}
