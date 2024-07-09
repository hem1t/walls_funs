use std::{collections::HashMap, fs::{self, read}, path::PathBuf};
use tinyjson::JsonValue;

#[derive(Debug, PartialEq)]
struct Settings {
    api_key: Option<String>,
    wall_cmd: Option<String>,
}

macro_rules! create {
    ($key:ident, $data:ident) => {
        let $key = $data.get(stringify!($key)).map(|s| { s.stringify().expect("failed to parse stringify!($key) value, from config!") });
    };
}

impl Settings {
    fn from_file(path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let file_data = fs::read_to_string(path)?;
        let json_data: JsonValue = file_data.parse().unwrap();
        let json_data: &HashMap<String, JsonValue> = json_data.get().unwrap();

        create!(api_key, json_data);
        create!(wall_cmd, json_data);

        Ok(
            Self {
                api_key,
                wall_cmd
            }
        )
    }
}

#[test]
fn read_example_config() {
    assert_eq!(
        Settings::from_file("config.json".into()).unwrap(),
        Settings {
            api_key: Some("hello I'm api_key".to_string()),
            wall_cmd: Some("nitrogen".to_string())
        });
}
