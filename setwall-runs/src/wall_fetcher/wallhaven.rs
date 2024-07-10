use serde::Deserialize;

use super::ImageFetcher;

pub(crate) struct Wallhaven {
    /// prefix for wallpaper names (before id).
    prefix: String,
    api_key: String,
    id: String,
    image_url: String,
    image_format: String,
}

impl ImageFetcher for Wallhaven {
    fn new(settings: &crate::settings::Settings, url: &String) -> Self {

        let id = unimplemented!();
        let image_url = unimplemented!();
        let image_format = unimplemented!();

        Self {
            prefix: settings.wallhaven_prefix_or("wallhaven"),
            api_key: settings.wallhaven_api_key_or(""),
            id,
            image_url,
            image_format
        }
    }

    fn get_url(&self) -> &String {
        &self.image_url
    }

    fn get_prefix(&self) -> &String {
        &self.prefix
    }

    fn get_id(&self) -> &String {
        &self.id
    }

    fn get_format(&self) -> &String {
        &self.image_format
    }
}

impl Wallhaven {
    /// give out (id, image_url, image_format)
    fn read_url(key: &String, url: &String) -> (String, String, String) {
        let id_part = url.split('/').last().unwrap();
        unimplemented!()
    }
}

fn handle_connection(mut url: String) {
    if url.contains("wallhaven.cc") && !url.contains("full") {
        let id = url.split('/').last().unwrap();
        let api_key = "1me7AqGu5OXo9wsduuiavxLgzT4w3Xxl";
        let api_url = format!("https://wallhaven.cc/api/v1/w/{id}?apikey={api_key}");
        if let Some(u) = url_from_wallhaven(api_url) {
            url = u;
        } else {
            // notify(dbg!("404: Wallhaven"));
        }
    }
}

#[derive(Deserialize)]
struct PathData {
    path: String,
}

fn url_from_wallhaven(api_url: String) -> Option<String> {
    if let Ok(response) = reqwest::blocking::get(api_url) {
        if response.status() == 200 {
            return Some(
                response
                    .json::<PathData>()
                    .expect("Image path failed in parsing!")
                    .path,
            );
        }
    }
    None
}
