use std::path::PathBuf;
use std::fs::{self};

use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub(crate) struct WallhavenSettings {
    pub api_key: Option<String>,
    pub prefix: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub(crate) struct Settings {
    pub wallhaven: Option<WallhavenSettings>,
    pub wall_cmd: String,
}

impl Settings {
    pub fn get_wallhaven_settings(&self) -> (String, String) {
        self.wallhaven.as_ref().map(|w| {
            (
                w.api_key.clone().unwrap_or("".to_string()),
                w.prefix.clone().unwrap_or("wallhaven".to_string()),
            )
        }).unwrap()
    }
}


impl Settings {
    pub fn from_file(path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let file_data = fs::read_to_string(path)?;
        Ok(toml::from_str::<Settings>(&file_data)?)
    }
}

#[test]
fn read_example_config() {
    assert_eq!(
        Settings::from_file("config.toml".into()).unwrap(),
        Settings {
            wallhaven: Some(WallhavenSettings {
                api_key: Some("hello I'm api_key".to_string()),
                prefix: Some("wallhaven".to_string())
            }),
            wall_cmd: "nitrogen".to_string()
        }
    );
}
