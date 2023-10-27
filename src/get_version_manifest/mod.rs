use serde::{Deserialize, Serialize};
use serde_json;
use reqwest::get;

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionManifest {
    latest: Latest,
    versions: Vec<Version>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Latest {
    release: String,
    snapshot: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Version {
    id: String,
    #[serde(rename = "type")]
    type_: String,
    url: String,
    time: String,
    #[serde(rename = "releaseTime")]
    release_time: String,
}

pub async fn get_version_manifest() -> Result<VersionManifest, Box<dyn std::error::Error>>
{
    const VERSION_MANIFEST_URL: &str = "https://launchermeta.mojang.com/mc/game/version_manifest.json";

    let version_manifest_body = get(VERSION_MANIFEST_URL).await?.text().await?;
    let version_manifest_json: VersionManifest = serde_json::from_str(version_manifest_body.as_str())?;

    return Ok(version_manifest_json);
}

pub async fn get_version_url(version_manifest: VersionManifest, id: &str) -> String
{
    version_manifest.versions.iter().cloned().find(|version| version.id == *id).unwrap().url
}