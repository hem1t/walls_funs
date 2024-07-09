use std::path::PathBuf;
use std::fs::{self};

use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
struct Settings {
    api_key: Option<String>,
    wall_cmd: String,
}

impl Settings {
    fn from_file(path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let file_data = fs::read_to_string(path)?;
        Ok(toml::from_str::<Settings>(&file_data)?)
    }
}

#[test]
fn read_example_config() {
    assert_eq!(
        Settings::from_file("config.toml".into()).unwrap(),
        Settings {
            api_key: Some("hello I'm api_key".to_string()),
            wall_cmd: "nitrogen".to_string()
        }
    );
}
