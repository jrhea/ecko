use eyre::Result;
use ipfs_api_backend_hyper::{IpfsApi, IpfsClient};
use std::path::Path;
use tracing::info;
use url::Url;
use std::io::Cursor;

#[derive(Clone)]
pub struct IpfsService {
    client: IpfsClient,
}

impl IpfsService {
    pub fn new(ipfs_api_url: &str) -> Result<Self> {
        let url = Url::parse(ipfs_api_url)?;
        Ok(Self {
            client: IpfsClient::default()
        })
    }

    /// Pin a file to IPFS and return its CID
    pub async fn pin_file(&self, file_path: &Path) -> Result<String> {
        let file = tokio::fs::read(file_path).await?;
        let cursor = Cursor::new(file);
        let response = self.client.add(cursor).await?;
        info!("File added to IPFS with hash: {}", response.hash);
        
        // Pin the file
        self.client.pin_add(&response.hash, true).await?;
        info!("File pinned successfully: {}", response.hash);
        
        Ok(response.hash)
    }

    /// Pin content by CID
    pub async fn pin_cid(&self, cid: &str) -> Result<()> {
        self.client.pin_add(cid, true).await?;
        info!("Content pinned successfully: {}", cid);
        Ok(())
    }

    /// Unpin content by CID
    pub async fn unpin_cid(&self, cid: &str) -> Result<()> {
        self.client.pin_rm(cid, true).await?;
        info!("Content unpinned successfully: {}", cid);
        Ok(())
    }

    /// List all pinned content
    pub async fn list_pins(&self) -> Result<Vec<String>> {
        let pins = self.client.pin_ls(None, None).await?;
        Ok(pins.keys.keys().cloned().collect())
    }
}
