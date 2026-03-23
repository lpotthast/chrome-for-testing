use chrome_for_testing::{
    DownloadsByPlatform, Error, KnownGoodVersions, LastKnownGoodVersions, Platform, Version,
};

#[tokio::test]
async fn test_last_known_good_versions() -> Result<(), Error> {
    let client = reqwest::Client::new();
    let versions = LastKnownGoodVersions::fetch(&client).await?;

    // Get the stable channel version.
    if let Some(stable) = versions.stable() {
        println!("Latest stable version: {}", stable.version);

        // Print download URLs for all platforms.
        for download in &stable.downloads.chrome {
            println!(
                "Available for platform '{}' at: {}",
                download.platform, download.url
            );
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_known_good_versions() -> Result<(), Error> {
    let client = reqwest::Client::new();
    let versions = KnownGoodVersions::fetch(&client).await?;

    println!("Found {} Chrome versions", versions.versions.len());

    // Find a specific version.
    let target_version: Version = "131.0.6778.204".parse().unwrap();
    if let Some(version) = versions
        .versions
        .iter()
        .find(|v| v.version == target_version)
    {
        println!(
            "Found version {}: revision {}",
            version.version, version.revision
        );

        let current_platform = Platform::detect().expect("Running on supported platform.");

        println!(
            "Chrome downloads available for {} platforms",
            version.downloads.chrome.len()
        );
        if let Some(download) = version.downloads.chrome.for_platform(current_platform) {
            println!(
                "Found Chrome download URL for current platform '{current_platform}': {}",
                download.url
            );
        }

        // ChromeDriver downloads may not be available for older versions.
        if let Some(chromedriver_downloads) = &version.downloads.chromedriver {
            println!(
                "ChromeDriver available for {} platforms",
                chromedriver_downloads.len()
            );
            if let Some(download) = chromedriver_downloads.for_platform(current_platform) {
                println!(
                    "Found ChromeDriver download for current platform '{current_platform}': {}",
                    download.url
                );
            }
        } else {
            println!("No ChromeDriver downloads available for this version");
        }
    }

    Ok(())
}
