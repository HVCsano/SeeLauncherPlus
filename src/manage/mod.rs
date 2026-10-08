use std::process::Command;

pub mod folders;
pub mod seelauncher;

pub fn launch_seemta() {
    Command::new("powershell.exe")
        .arg("Start-Process -FilePath 'seelauncherplus.exe' -ArgumentList '-run'")
        .spawn()
        .unwrap();
}
