use std::{fs, path::PathBuf};
use paste::paste;
use getset::Getters;

use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub(crate) struct WallhavenSettings {
    pub api_key: Option<String>,
    pub prefix: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize, Getters)]
pub(crate) struct Settings {
    pub wallhaven: Option<WallhavenSettings>,
    #[getset(get = "pub")]
    pub wall_cmd: String,
    #[getset(get = "pub")]
    pub dir_path: String,
}

macro_rules! create_getter {
    ($serv:ident.$set:ident) => {
        paste! {
            pub fn [<$serv _ $set _or>]<'a>(&'a self, or: &'a str) -> String {
                self.$serv
                    .as_ref()
                    .and_then(|s| s.$set.as_ref().map(|s| s.clone()))
                    .unwrap_or(or.to_owned())
            }
        }
    };
}

impl Settings {
    create_getter!(wallhaven.prefix);
    create_getter!(wallhaven.api_key);
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
            wall_cmd: "nitrogen".to_string(),
            dir_path: "/path/to/images".to_string()

        }
    );
}

#[test]
fn default_settings() {
    use resolve_path::PathResolveExt;
    assert!(
        Settings::from_file("~/.config/wall_funs/config.toml".resolve().to_path_buf()).is_ok(),
    );
}

