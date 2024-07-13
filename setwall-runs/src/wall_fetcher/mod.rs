use std::{fs, io};

use crate::settings::Settings;

pub(crate) mod wallhaven;

pub(crate) enum DownloadError {
    FetchError(reqwest::Error),
    WriteError(io::Error),
}

pub(crate) trait ImageFetcher {
    fn get_url(&self) -> &String;

    fn get_prefix(&self) -> &String;

    fn get_id(&self) -> &String;

    fn get_format(&self) -> &String;

    fn download_image(&self, dir_path: String) -> Result<(), DownloadError> {
        let resp = match reqwest::blocking::get(self.get_url()) {
            Ok(r) => r,
            Err(e) => return Err(DownloadError::FetchError(e))
        };
        let bytes = match resp.bytes() {
            Ok(b) => b,
            Err(e) => return Err(DownloadError::FetchError(e))
        };

        self.write_to_file(dir_path, bytes)?;
        Ok(())
    }

    fn write_to_file<S: AsRef<[u8]>>(&self, dir_path: String, contents: S) -> Result<(), DownloadError> {
        let path = format!(
            "{dir_path}/{}-{}.{}",
            self.get_prefix(),
            self.get_id(),
            self.get_format()
        );

        if let Err(err) = fs::write(path, contents) {
            return Err(DownloadError::WriteError(err));
        }

        Ok(())
    }
}
