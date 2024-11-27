use crate::utils;
use reqwest::header;
use std::io::Cursor;
use tokio::io::copy;
use tracing::debug;

#[derive(Debug, Clone)]
pub struct IconStore {}

pub enum IconStoreError {
    FileSystemFailToWrite,
    UnableToSendRequest,
    UnableToParseResponse,
}

impl IconStore {
    pub fn new() -> Self {
        return IconStore {};
    }

    pub async fn init(&self) -> Result<(), IconStoreError> {
        tokio::fs::create_dir("./icons")
            .await
            .map_err(|_| IconStoreError::FileSystemFailToWrite)?;

        Ok(())
    }

    pub async fn find_or_gather(&self, domain: &str) -> Result<Vec<u8>, IconStoreError> {
        match tokio::fs::read(format!("./icons/{}.ico", utils::hash_domain(domain))).await {
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

        let mut file =
            tokio::fs::File::create(format!("./icons/{}.ico", utils::hash_domain(domain)))
                .await
                .map_err(|_| IconStoreError::FileSystemFailToWrite)?;

        let mut content = Cursor::new(&req);
        copy(&mut content, &mut file)
            .await
            .map_err(|_| IconStoreError::FileSystemFailToWrite)?;

        Ok(req.to_vec())
    }
}
