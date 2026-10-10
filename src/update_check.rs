use seelauncherplus_lib::WEB_CLIENT;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GitHubResponse {
    pub tag_name: String,
}

pub fn get_latest_tag() -> String {
    let req = WEB_CLIENT
        .get("https://api.github.com/repos/hvcsano/seelauncherplus/releases/latest")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/155.0.0.0 Safari/537.36")
        .send();
    if req.is_err() {
        return "".to_string();
    }
    let req = req.unwrap();

    let data = req.json::<GitHubResponse>();

    if data.is_err() {
        return "".to_string();
    }

    return data.unwrap().tag_name;
}
