# chrome-for-testing

Provides serde-enabled type definitions covering the **chrome-for-testing** JSON API responses,
and programmatic access to the API endpoints through `reqwest`, allowing you to fetch information
about available Chrome and ChromeDriver versions for automated testing.

## Links

- **Blog post**: [Chrome for Testing announcement](https://developer.chrome.com/blog/chrome-for-testing)
- **Live API**: [Chrome for Testing availability](https://googlechromelabs.github.io/chrome-for-testing/)
- **API documentation**: [JSON API endpoints](https://github.com/GoogleChromeLabs/chrome-for-testing#json-api-endpoints)
- **Crate documentation**: [docs.rs](https://docs.rs/chrome-for-testing)

## Related Crates

### chrome-for-testing-manager

You may want to check out [chrome-for-testing-manager](https://github.com/lpotthast/chrome-for-testing-manager), a
crate building upon this one to allow easy selection and installation of chrome-for-testing versions.\
It also comes with support for the [thirtyfour](https://crates.io/crates/thirtyfour) WebDriver crate.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
chrome-for-testing = "0.3"
```

Or use `cargo add`:

```shell
cargo add chrome-for-testing
```

## Features

- **Type-safe API access** - Serde-enabled type definitions for all API responses.
- **Async support** - Built on `reqwest` for non-blocking HTTP requests.
- **Provides access to the following APIs**:
    - `KnownGoodVersions` - Get all historical Chrome versions.
    - `LastKnownGoodVersions` - Get latest versions for each release channel.
- **Platform detection** - Automatically detect the current platform (os/arch) to filter responses.
- **ChromeDriver utilities** - Additional tools for ChromeDriver configuration.

## Usage

### Getting the "Last Known Good Versions"

Use this API if you just want to know the latest version of one particular (or multiple) release channels.

```rust
use chrome_for_testing::{Error, LastKnownGoodVersions};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = reqwest::Client::new();
    let versions = LastKnownGoodVersions::fetch(&client).await?;

    // Get the stable channel version.
    if let Some(stable) = versions.stable() {
        println!("Latest stable version: {}", stable.version);

        // Print download URLs for all platforms.
        for download in &stable.downloads.chrome {
            println!("Available for platform '{}' at: {}", download.platform, download.url);
        }
    }

    Ok(())
}
```

### Getting all "Known Good Versions"

Use this API if you just want to know all historical version. Particularly useful if you plan to run a fixed version.

This example also shows how the `Platform` type can be used to filter available download URLs for the current platform.

```rust
use chrome_for_testing::{DownloadsByPlatform, Error, KnownGoodVersions, Platform, Version};

#[tokio::main]
async fn main() -> Result<(), Error> {
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
```

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
