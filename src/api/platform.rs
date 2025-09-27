use crate::error::Error;
use crate::error::Result;
use serde::Deserialize;
use std::borrow::Cow;
use std::env::consts;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum Platform {
    #[serde(rename = "linux64")]
    Linux64,
    #[serde(rename = "mac-arm64")]
    MacArm64,
    #[serde(rename = "mac-x64")]
    MacX64,
    #[serde(rename = "win32")]
    Win32,
    #[serde(rename = "win64")]
    Win64,
}

impl Display for Platform {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Platform::Linux64 => "linux64",
            Platform::MacArm64 => "mac-arm64",
            Platform::MacX64 => "mac-x64",
            Platform::Win32 => "win32",
            Platform::Win64 => "win64",
        })
    }
}

impl Platform {
    /// Detect the platform identifier that should be used for the current system.
    pub fn detect() -> Result<Platform> {
        match consts::OS {
            os @ "windows" => match consts::ARCH {
                "x86" => Ok(Platform::Win32),
                "x86_64" => Ok(Platform::Win64),
                arch => Err(Error::UnsupportedPlatform {
                    os: Cow::Borrowed(os),
                    arch: Cow::Borrowed(arch),
                }),
            },
            os @ "linux" => match consts::ARCH {
                "x86_64" => Ok(Platform::Linux64),
                arch => Err(Error::UnsupportedPlatform {
                    os: Cow::Borrowed(os),
                    arch: Cow::Borrowed(arch),
                }),
            },
            os @ "macos" => match consts::ARCH {
                "x86_64" => Ok(Platform::MacX64),
                "arm" | "aarch64" => Ok(Platform::MacArm64),
                arch => Err(Error::UnsupportedPlatform {
                    os: Cow::Borrowed(os),
                    arch: Cow::Borrowed(arch),
                }),
            },
            os => Err(Error::UnsupportedPlatform {
                os: Cow::Borrowed(os),
                arch: Cow::Borrowed(consts::ARCH),
            }),
        }
    }

    /// Filename of the chrome binary.
    pub fn chrome_binary_name(self) -> &'static str {
        match self {
            Platform::Linux64 | Platform::MacX64 => "chrome",
            Platform::MacArm64 => "Google Chrome for Testing.app",
            Platform::Win32 | Platform::Win64 => "chrome.exe",
        }
    }

    /// Filename of the chromedriver binary.
    pub fn chromedriver_binary_name(self) -> &'static str {
        match self {
            Platform::Linux64 | Platform::MacX64 | Platform::MacArm64 => "chromedriver",
            Platform::Win32 | Platform::Win64 => "chromedriver.exe",
        }
    }
}
