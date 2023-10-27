
mod get_version_manifest;
mod get_version_info;

use get_version_manifest::{get_version_manifest, get_version_url};
use get_version_info::{get_version};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let url = get_version_url(get_version_manifest().await?, "1.20.2").await;

    println!("{:?}", get_version(url.as_str()).await?);
    Ok(())
}




