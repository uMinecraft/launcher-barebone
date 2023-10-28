use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Version {
    arguments: Arguments,
    #[serde(rename = "assetIndex")]
    asset_index: AssetIndex,
    assets: String,
    #[serde(rename = "complianceLevel")]
    compliance_level: i64,
    downloads: Downloads,
    id: String,
    #[serde(rename = "javaVersion")]
    java_version: JavaVersion,
    libraries: Vec<Library>,
    logging: Logging,
    #[serde(rename = "mainClass")]
    main_class: String,
    #[serde(rename = "minimumLauncherVersion")]
    minimum_launcher_version: i64,
    #[serde(rename = "releaseTime")]
    release_time: String,
    time: String,
    #[serde(rename = "type")]
    type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Arguments {
    game: Vec<Value>,
    jvm: (Jvm, Jvm2, Jvm3, String, String, String, String, String, String, String, String),
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Jvm {
    rules: Vec<Rule>,
    value: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Rule {
    action: String,
    os: Os,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Os {
    name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Jvm2 {
    rules: Vec<Rule2>,
    value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Rule2 {
    action: String,
    os: Os2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Os2 {
    name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Jvm3 {
    rules: Vec<Rule3>,
    value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Rule3 {
    action: String,
    os: Os3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Os3 {
    arch: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct AssetIndex {
    id: String,
    sha1: String,
    size: i64,
    #[serde(rename = "totalSize")]
    total_size: i64,
    url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Downloads {
    client: Client,
    client_mappings: ClientMappings,
    server: Server,
    server_mappings: ServerMappings,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Client {
    sha1: String,
    size: i64,
    url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct ClientMappings {
    sha1: String,
    size: i64,
    url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Server {
    sha1: String,
    size: i64,
    url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct ServerMappings {
    sha1: String,
    size: i64,
    url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct JavaVersion {
    component: String,
    #[serde(rename = "majorVersion")]
    major_version: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Library {
    downloads: Downloads2,
    name: String,
    #[serde(default)]
    rules: Vec<Rule4>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Downloads2 {
    artifact: Artifact,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Artifact {
    pub path: String,
    sha1: String,
    size: i64,
    url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Rule4 {
    action: String,
    os: Os4,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Os4 {
    name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Logging {
    client: Client2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Client2 {
    argument: String,
    file: File,
    #[serde(rename = "type")]
    type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct File {
    id: String,
    sha1: String,
    size: i64,
    url: String,
}


pub async fn get_version(url: &str) -> reqwest::Result<Version> {
    let response = reqwest::get(url).await?;
    let version: Version = response.json().await?;
    Ok(version)
}