use std::sync::Arc;

use anyhow::{bail, Context, Result};
use serde::Deserialize;

use crate::settings::Settings;

use super::ImageFetcher;

pub(crate) struct Wallhaven {
    /// prefix for wallpaper names (before id).
    prefix: String,
    id: String,
    image_url: String,
    image_format: String,
    pub settings: Arc<Settings>,
}

impl Wallhaven {
    pub fn new(settings: Arc<Settings>, url: &str) -> Result<Self> {
        let prefix = settings.wallhaven_prefix_or("wallhaven");
        let api_key = settings.wallhaven_api_key_or("");
        let (id, image_url, image_format) =
            Wallhaven::read_link_url(&api_key.to_string(), &url.to_string())?;

        Ok(Self {
            prefix,
            id,
            image_url,
            image_format,
            settings
        })
    }
}

impl ImageFetcher for Wallhaven {
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

    fn settings(&self) -> &Settings {
        &self.settings
    }
}

impl Wallhaven {
    /// give out (id, image_url, image_format)
    fn read_link_url(key: &String, url: &String) -> Result<(String, String, String)> {
        let id = url.split('/').last().unwrap().to_owned();
        let data = data_from_api(&id.to_string(), key)?;
        Ok((id, data.get_path(), data.get_filetype()))
    }
}

#[derive(Deserialize)]
struct APIData {
    data: WallAPIData,
}

#[derive(Deserialize)]
struct WallAPIData {
    path: String,
    file_type: String,
}

impl APIData {
    fn get_path(&self) -> String {
        self.data.path.clone()
    }

    fn get_filetype(&self) -> String {
        {
            if self.data.file_type == "image/jpeg" {
                "jpg"
            } else if self.data.file_type == "image/png" {
                "png"
            } else {
                ""
            }
        }
        .to_string()
    }
}

fn data_from_api(id: &String, key: &String) -> Result<APIData> {
    let api_url = dbg!(format!("https://wallhaven.cc/api/v1/w/{id}?apikey={key}"));
    let response = reqwest::blocking::get(api_url.clone())?;

    if response.status() == 200 {
        return Ok(response.json::<APIData>().with_context(|| {
            format!(
                "Fetch data from {}, couldn't be parsed!",
                api_url.split_once('?').unwrap().0
            )
        })?);
    } else {
        bail!(
            "Wallhaven api response code failed with status code of \"{}\"",
            response.status()
        );
    }
}
