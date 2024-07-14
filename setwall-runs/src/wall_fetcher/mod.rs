use crate::settings::Settings;
use anyhow::Result;
use std::{fs, io, path::PathBuf, process::Command};
use thiserror::Error;

pub(crate) mod wallhaven;

#[derive(Debug, Error)]
pub(crate) enum DownloadError {
    #[error("Couldn't load from {0:?}")]
    FetchError(#[from] reqwest::Error),
    #[error("Couldn't write to file {0:?}")]
    WriteError(#[from] io::Error),
}

pub(crate) trait ImageFetcher {
    fn get_url(&self) -> &String;

    fn get_prefix(&self) -> &String;

    fn get_id(&self) -> &String;

    fn get_format(&self) -> &String;

    fn settings(&self) -> &Settings;

    fn file_path(&self) -> PathBuf {
        let dir_path = self.settings().dir_path();
        PathBuf::from(format!(
            "{dir_path}/{}-{}.{}",
            self.get_prefix(),
            self.get_id(),
            self.get_format()
        ))
    }

    fn download_image(&self) -> Result<&Self, DownloadError> {
        let path = self.file_path();
        if path.exists() {
            return Ok(self);
        }

        let resp = match reqwest::blocking::get(self.get_url()) {
            Ok(r) => r,
            Err(e) => return Err(DownloadError::FetchError(e)),
        };
        let bytes = match resp.bytes() {
            Ok(b) => b,
            Err(e) => return Err(DownloadError::FetchError(e)),
        };

        if let Err(err) = fs::write(&path, bytes) {
            return Err(DownloadError::WriteError(err));
        }

        Ok(self)
    }

    fn setting_wall(&self) -> Result<()> {
        let path = self.file_path();
        let _out = Command::new(self.settings().wall_cmd.to_string())
            .arg(path)
            .output()
            .expect("failed setting");
        Ok(())
    }
}
