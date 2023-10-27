use serde::{Deserialize, Serialize};
use serde_json;
use reqwest::get;

#[derive(Debug, Serialize, Deserialize)]
pub struct MinecraftVersion {
    arguments: Arguments,
    #[serde(rename = "assetIndex")]
    asset_index: AssetIndex,
    assets: String,
    downloads: Downloads,
    id: String,
    #[serde(rename = "type")]
    type_: String,
    #[serde(rename = "releaseTime")]
    release_time: String,
    time: String,
    #[serde(rename = "mainClass")]
    main_class: String,
    #[serde(rename = "minimumLauncherVersion")]
    minimum_launcher_version: i32,
    logging: Logging,
    libraries: Library
}

#[derive(Debug, Serialize, Deserialize)]
struct Library {
    downloads: LibraryDownload,
    name: String,
    rules: Rules,
}

#[derive(Debug, Serialize, Deserialize)]
struct LibraryDownload {
    artifact: Artifact,
}

#[derive(Debug, Serialize, Deserialize)]
struct Rules {
    action: String,
    os: Os,
}

#[derive(Debug, Serialize, Deserialize)]
struct Os {
    name: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Artifact {
    path: String,
    sha1: String,
    size: i64,
    url: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Arguments {
    game: Vec<serde_json::Value>,
    jvm: Vec<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AssetIndex {
    id: String,
    sha1: String,
    size: i64,
    #[serde(rename = "totalSize")]
    total_size: i64,
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Downloads {
    client: DownloadEntry,
    client_mappings: DownloadEntry,
    server: DownloadEntry,
    server_mappings: DownloadEntry,
}

#[derive(Debug, Serialize, Deserialize)]
struct DownloadEntry {
    sha1: String,
    size: i64,
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Logging {
    client: LoggingClient,
}

#[derive(Debug, Serialize, Deserialize)]
struct LoggingClient {
    argument: String,
    file: LoggingFile,
    #[serde(rename = "type")]
    type_: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LoggingFile {
    id: String,
    sha1: String,
    size: i64,
    url: String,
}

pub async fn get_version(url: &str) -> Result<MinecraftVersion, Box<dyn std::error::Error>>
{
    let body = get(url).await?.text().await?;
    let minecraft_version: MinecraftVersion = serde_json::from_str(body.as_str())?;
    return  Ok(minecraft_version);
}