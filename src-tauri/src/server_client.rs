use reqwest::Client;
use serde::Deserialize;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::error::Error;
use tokio::io::AsyncWriteExt;
use tokio::fs as tokio_fs;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
struct FileEntry {
    name: String,
    is_dir: bool,
}

async fn fetch_directory(client: &Client, base_url: &str) -> Result<Vec<FileEntry>, Box<dyn Error>> {
    let response = client.get(base_url).send().await?;
    let body = response.text().await?;
    let entries: Vec<FileEntry> = serde_json::from_str(&body)?;
    Ok(entries)
}

async fn download_file(client: &Client, file_url: &str, save_path: &Path) -> Result<(), Box<dyn Error>> {
    let mut response = client.get(file_url).send().await?;
    let mut file = tokio_fs::File::create(save_path).await?;

    while let Some(chunk) = response.chunk().await? {
        file.write_all(&chunk).await?;
    }

    Ok(())
}

async fn process_directory(
    client: Arc<Client>,
    base_url: &str,
    local_path: &Path,
) -> Result<(), Box<dyn Error>> {
    let entries = fetch_directory(&client, base_url).await?;

    for entry in entries {
        let entry_url = format!("{}/{}", base_url, entry.name);
        let entry_path = local_path.join(&entry.name);

        if entry.is_dir {
            // Create directory locally
            tokio_fs::create_dir_all(&entry_path).await?;
            // Recursively process the directory
            Box::pin(process_directory(client.clone(), &entry_url, &entry_path)).await?;
        } else {
            // Download the file
            download_file(&client, &entry_url, &entry_path).await?;
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn get_host_linked_paths(
    base_url: String,
    local_path: String,
) -> Result<String, String> {
    // Ensure the local directory exists
    let local_path = Path::new(&local_path);
    if !local_path.exists() {
        return Err(format!("The specified path '{}' does not exist.", local_path.display()));
    }

    // Create an Arc<Client> so it can be shared across async tasks
    let client = Arc::new(Client::new());

    // Start processing the directory
    match process_directory(client.clone(), &base_url, local_path).await {
        Ok(_) => Ok(format!("Files from '{}' downloaded successfully.", base_url)),
        Err(e) => Err(format!("Error during download: {}", e)),
    }
}
