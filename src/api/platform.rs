use crate::error::Error;
use rootcause::{Report, report};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::env::consts;
use std::fmt::{Display, Formatter};
use std::path::Path;
use std::str::FromStr;

/// Error returned when parsing a platform string fails.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("Unknown platform: '{value}'. Expected one of: linux64, mac-arm64, mac-x64, win32, win64")]
pub struct ParsePlatformError {
    value: String,
}

/// Supported platforms for Chrome and `ChromeDriver` downloads.
///
/// This site <https://googlechromelabs.github.io/chrome-for-testing/> show the platform names
/// defined here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

impl FromStr for Platform {
    type Err = Report<ParsePlatformError>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "linux64" => Ok(Platform::Linux64),
            "mac-arm64" => Ok(Platform::MacArm64),
            "mac-x64" => Ok(Platform::MacX64),
            "win32" => Ok(Platform::Win32),
            "win64" => Ok(Platform::Win64),
            _ => Err(report!(ParsePlatformError {
                value: s.to_owned(),
            })),
        }
    }
}

impl Platform {
    /// Detect the platform identifier that should be used for the current system.
    ///
    /// # Errors
    ///
    /// Returns an error if the current OS/architecture combination is not supported.
    pub fn detect() -> crate::Result<Platform> {
        match consts::OS {
            os @ "windows" => match consts::ARCH {
                "x86" => Ok(Platform::Win32),
                "x86_64" => Ok(Platform::Win64),
                arch => Err(report!(Error::UnsupportedPlatform {
                    os: Cow::Borrowed(os),
                    arch: Cow::Borrowed(arch),
                })),
            },
            os @ "linux" => match consts::ARCH {
                "x86_64" => Ok(Platform::Linux64),
                arch => Err(report!(Error::UnsupportedPlatform {
                    os: Cow::Borrowed(os),
                    arch: Cow::Borrowed(arch),
                })),
            },
            os @ "macos" => match consts::ARCH {
                "x86_64" => Ok(Platform::MacX64),
                "arm" | "aarch64" => Ok(Platform::MacArm64),
                arch => Err(report!(Error::UnsupportedPlatform {
                    os: Cow::Borrowed(os),
                    arch: Cow::Borrowed(arch),
                })),
            },
            os => Err(report!(Error::UnsupportedPlatform {
                os: Cow::Borrowed(os),
                arch: Cow::Borrowed(consts::ARCH),
            })),
        }
    }

    /// Filename of the Chrome executable.
    #[must_use]
    pub fn chrome_executable_name(self) -> &'static str {
        match self {
            Platform::Linux64 => "chrome",
            Platform::MacArm64 | Platform::MacX64 => "Google Chrome for Testing",
            Platform::Win32 | Platform::Win64 => "chrome.exe",
        }
    }

    /// Relative path of the Chrome executable inside the unpacked Chrome archive.
    #[must_use]
    pub fn chrome_executable_path(self) -> &'static Path {
        match self {
            Platform::Linux64 => Path::new("chrome-linux64/chrome"),
            Platform::MacArm64 => Path::new(
                "chrome-mac-arm64/Google Chrome for Testing.app/Contents/MacOS/Google Chrome for Testing",
            ),
            Platform::MacX64 => Path::new(
                "chrome-mac-x64/Google Chrome for Testing.app/Contents/MacOS/Google Chrome for Testing",
            ),
            Platform::Win32 => Path::new("chrome-win32/chrome.exe"),
            Platform::Win64 => Path::new("chrome-win64/chrome.exe"),
        }
    }

    /// Filename of the `ChromeDriver` executable.
    #[must_use]
    pub fn chromedriver_executable_name(self) -> &'static str {
        match self {
            Platform::Linux64 | Platform::MacX64 | Platform::MacArm64 => "chromedriver",
            Platform::Win32 | Platform::Win64 => "chromedriver.exe",
        }
    }

    /// Relative path of the `ChromeDriver` executable inside the unpacked `ChromeDriver` archive.
    #[must_use]
    pub fn chromedriver_executable_path(self) -> &'static Path {
        match self {
            Platform::Linux64 => Path::new("chromedriver-linux64/chromedriver"),
            Platform::MacArm64 => Path::new("chromedriver-mac-arm64/chromedriver"),
            Platform::MacX64 => Path::new("chromedriver-mac-x64/chromedriver"),
            Platform::Win32 => Path::new("chromedriver-win32/chromedriver.exe"),
            Platform::Win64 => Path::new("chromedriver-win64/chromedriver.exe"),
        }
    }

    /// Filename of the Chrome Headless Shell executable.
    #[must_use]
    pub fn chrome_headless_shell_executable_name(self) -> &'static str {
        match self {
            Platform::Linux64 | Platform::MacX64 | Platform::MacArm64 => "chrome-headless-shell",
            Platform::Win32 | Platform::Win64 => "chrome-headless-shell.exe",
        }
    }

    /// Relative path of the Chrome Headless Shell executable inside the unpacked
    /// Chrome Headless Shell archive.
    #[must_use]
    pub fn chrome_headless_shell_executable_path(self) -> &'static Path {
        match self {
            Platform::Linux64 => Path::new("chrome-headless-shell-linux64/chrome-headless-shell"),
            Platform::MacArm64 => {
                Path::new("chrome-headless-shell-mac-arm64/chrome-headless-shell")
            }
            Platform::MacX64 => Path::new("chrome-headless-shell-mac-x64/chrome-headless-shell"),
            Platform::Win32 => Path::new("chrome-headless-shell-win32/chrome-headless-shell.exe"),
            Platform::Win64 => Path::new("chrome-headless-shell-win64/chrome-headless-shell.exe"),
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

#[cfg(test)]
mod tests {
    use super::*;
    use assertr::prelude::*;

    #[test]
    fn parse_to_string_round_trip() {
        let platforms = [
            ("linux64", Platform::Linux64),
            ("mac-arm64", Platform::MacArm64),
            ("mac-x64", Platform::MacX64),
            ("win32", Platform::Win32),
            ("win64", Platform::Win64),
        ];
        for (s, expected) in platforms {
            assert_that!(s.parse::<Platform>())
                .is_ok()
                .is_equal_to(expected);
            assert_that!(expected.to_string()).is_equal_to(s);
        }
    }

    #[test]
    fn parse_invalid_variant_fails() {
        assert_that!("Linux64".parse::<Platform>()).is_err();
        assert_that!("unknown".parse::<Platform>()).is_err();
    }

    #[test]
    fn executable_path_file_names_match_executable_names() {
        let platforms = [
            Platform::Linux64,
            Platform::MacArm64,
            Platform::MacX64,
            Platform::Win32,
            Platform::Win64,
        ];

        for platform in platforms {
            assert_that!(
                platform
                    .chrome_executable_path()
                    .file_name()
                    .and_then(|it| it.to_str())
            )
            .is_equal_to(Some(platform.chrome_executable_name()));
            assert_that!(
                platform
                    .chromedriver_executable_path()
                    .file_name()
                    .and_then(|it| it.to_str())
            )
            .is_equal_to(Some(platform.chromedriver_executable_name()));
            assert_that!(
                platform
                    .chrome_headless_shell_executable_path()
                    .file_name()
                    .and_then(|it| it.to_str())
            )
            .is_equal_to(Some(platform.chrome_headless_shell_executable_name()));
        }
    }

    #[test]
    fn serialized_value_matches_display_output() {
        assert_that!(serde_json::to_string(&Platform::Linux64).unwrap())
            .is_equal_to(String::from("\"linux64\""));
        assert_that!(serde_json::to_string(&Platform::MacArm64).unwrap())
            .is_equal_to(String::from("\"mac-arm64\""));
    }
}
