use std::sync::Arc;

use crc64fast::Digest;
use anyhow::Result;

use crate::settings::Settings;

pub(crate) struct General {
    url: String,
    prefix: String,
    id: String,
    format: String,
    settings: Arc<Settings>,
}

impl General {
    pub fn new(settings: &Arc<Settings>, url: &str) -> Result<Self> {
        let prefix = settings.general_prefix();
        // let (id, image_url, image_format) =;
    }
}

impl super::ImageFetcher for General {
    fn get_url(&self) -> &String {
        &self.url
    }

    fn get_prefix(&self) -> &String {
        &self.prefix
    }

    fn get_id(&self) -> &String {
        &self.id
    }

    fn get_format(&self) -> &String {
        &self.format
    }

    fn settings(&self) -> &crate::settings::Settings {
        &self.settings
    }
}

impl General {
    fn read_url(url: &str) -> Result<(String, String, String)> {
        // NOTE: needs a downloader
        unimplemented!()
    }

    fn figure_format(bytes: &[u8]) -> String {
        unimplemented!()
    }

    fn gen_id(bytes: &[u8]) -> String {
        let mut d = Digest::new();
        d.write(bytes);
        dbg!(d.sum64().to_string())
    }
}
