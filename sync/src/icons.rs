use crate::utils;
use reqwest::header;
use std::{
    io::{Cursor, ErrorKind},
    path::PathBuf,
};
use tokio::io::copy;
use tracing::debug;

#[derive(Debug, Clone)]
pub struct IconStore {
    base: PathBuf,
}

#[derive(Debug)]
pub enum IconStoreError {
    FileSystemFailToWrite,
    UnableToSendRequest,
    UnableToParseResponse,
}

impl IconStore {
    pub fn new() -> Self {
        IconStore {
            base: PathBuf::from("./icons"),
        }
    }

    pub fn new_with_base(base: PathBuf) -> Self {
        IconStore { base }
    }

    fn get_path(&self, domain: &str) -> PathBuf {
        self.base
            .join(PathBuf::from(utils::hash_domain(domain) + ".ico"))
    }

    pub async fn init(&self) -> Result<(), IconStoreError> {
        match tokio::fs::create_dir(&self.base).await {
            Ok(_) => Ok(()),
            Err(e) if e.kind() == ErrorKind::AlreadyExists => Ok(()),
            Err(_) => Err(IconStoreError::FileSystemFailToWrite),
        }
    }

    pub async fn find_or_gather(&self, domain: &str) -> Result<Vec<u8>, IconStoreError> {
        match tokio::fs::read(self.get_path(domain)).await {
            Ok(content) => Ok(content),
            Err(_) => self.gather(domain).await,
        }
    }

    pub async fn gather(&self, domain: &str) -> Result<Vec<u8>, IconStoreError> {
        debug!("Gathering icon for {}", domain);
        let client = reqwest::Client::builder()
            .user_agent(utils::USER_AGENT)
            .build()
            .unwrap();

        let req = client
            .get(format!("https://{domain}/favicon.ico"))
            .header(header::CONTENT_TYPE, "image/x-icon")
            .send()
            .await
            .map_err(|_| IconStoreError::UnableToSendRequest)?
            .bytes()
            .await
            .map_err(|_| IconStoreError::UnableToParseResponse)?;

        let mut file = tokio::fs::File::create(self.get_path(domain))
            .await
            .map_err(|_| IconStoreError::FileSystemFailToWrite)?;

        let mut content = Cursor::new(&req);
        copy(&mut content, &mut file)
            .await
            .map_err(|_| IconStoreError::FileSystemFailToWrite)?;

        Ok(req.to_vec())
    }
}
