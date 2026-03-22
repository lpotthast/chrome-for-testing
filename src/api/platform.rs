use crate::error::Error;
use crate::error::Result;
use serde::Deserialize;
use std::borrow::Cow;
use std::env::consts;
use std::fmt::{Display, Formatter};

/// Supported platforms for Chrome and `ChromeDriver` downloads.
///
/// This site <https://googlechromelabs.github.io/chrome-for-testing/> show the platform names
/// defined here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum Platform {
    /// Linux x64 platform.
    #[serde(rename = "linux64")]
    Linux64,

    /// macOS ARM64 platform (Apple Silicon).
    #[serde(rename = "mac-arm64")]
    MacArm64,

    /// macOS x64 platform (Intel).
    #[serde(rename = "mac-x64")]
    MacX64,

    /// Windows 32-bit platform.
    #[serde(rename = "win32")]
    Win32,

    /// Windows 64-bit platform.
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
    ///
    /// # Errors
    ///
    /// Returns an error if the current OS/architecture combination is not supported.
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
    #[must_use]
    pub fn chrome_binary_name(self) -> &'static str {
        match self {
            Platform::Linux64 | Platform::MacX64 => "chrome",
            Platform::MacArm64 => "Google Chrome for Testing.app",
            Platform::Win32 | Platform::Win64 => "chrome.exe",
        }
    }

    /// Filename of the chromedriver binary.
    #[must_use]
    pub fn chromedriver_binary_name(self) -> &'static str {
        match self {
            Platform::Linux64 | Platform::MacX64 | Platform::MacArm64 => "chromedriver",
            Platform::Win32 | Platform::Win64 => "chromedriver.exe",
        }
    }

    /// Tells whether this platform identifier references the Linux OS.
    #[must_use]
    pub fn is_linux(&self) -> bool {
        match self {
            Platform::Linux64 => true,
            Platform::MacArm64 | Platform::MacX64 | Platform::Win32 | Platform::Win64 => false,
        }
    }

    /// Tells whether this platform identifier references macOS.
    #[must_use]
    pub fn is_macos(&self) -> bool {
        match self {
            Platform::MacArm64 | Platform::MacX64 => true,
            Platform::Linux64 | Platform::Win32 | Platform::Win64 => false,
        }
    }

    /// Tells whether this platform identifier references the Windows OS.
    #[must_use]
    pub fn is_windows(&self) -> bool {
        match self {
            Platform::Win32 | Platform::Win64 => true,
            Platform::Linux64 | Platform::MacArm64 | Platform::MacX64 => false,
        }
    }
}
