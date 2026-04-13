use rootcause::{Report, report};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Debug, Display};
use std::str::FromStr;

/// Error returned when parsing a channel string fails.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("Invalid channel: '{value}'. Channel names must not be empty.")]
pub struct ParseChannelError {
    value: String,
}

/// Chrome release channel.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Channel {
    /// The stable release channel - the default Chrome version for general users.
    Stable,

    /// The beta release channel - preview of upcoming stable features.
    /// Less stable than stable, used for testing new functionality before it reaches stable.
    Beta,

    /// The dev release channel - early development builds with cutting-edge features.
    /// Less stable than beta, intended for developers and early adopters.
    Dev,

    /// The canary release channel - nightly builds with the absolute latest changes.
    /// Less stable than dev, highly experimental.
    Canary,

    /// An upstream channel name this crate does not know yet.
    Other(String),
}

impl Serialize for Channel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for Channel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(serde::de::Error::custom)
    }
}

impl Display for Channel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Channel::Stable => "Stable",
            Channel::Beta => "Beta",
            Channel::Dev => "Dev",
            Channel::Canary => "Canary",
            Channel::Other(name) => name,
        })
    }
}

impl FromStr for Channel {
    type Err = Report<ParseChannelError>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Stable" | "stable" => Ok(Channel::Stable),
            "Beta" | "beta" => Ok(Channel::Beta),
            "Dev" | "dev" => Ok(Channel::Dev),
            "Canary" | "canary" => Ok(Channel::Canary),
            "" => Err(report!(ParseChannelError {
                value: s.to_owned(),
            })),
            name => Ok(Channel::Other(name.to_owned())),
        }
    }
}

impl Channel {
    /// Returns whether this is one of the four channel names currently documented by Chrome for
    /// Testing.
    #[must_use]
    pub fn is_known(&self) -> bool {
        match self {
            Channel::Stable | Channel::Beta | Channel::Dev | Channel::Canary => true,
            Channel::Other(_) => false,
        }
    }

    /// Returns the raw upstream channel name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Channel::Stable => "Stable",
            Channel::Beta => "Beta",
            Channel::Dev => "Dev",
            Channel::Canary => "Canary",
            Channel::Other(name) => name,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assertr::prelude::*;

    fn channels() -> [(&'static str, Channel); 10] {
        [
            ("Stable", Channel::Stable),
            ("stable", Channel::Stable),
            ("Beta", Channel::Beta),
            ("beta", Channel::Beta),
            ("Dev", Channel::Dev),
            ("dev", Channel::Dev),
            ("Canary", Channel::Canary),
            ("canary", Channel::Canary),
            ("Unknown", Channel::Other("Unknown".to_owned())),
            ("unknown", Channel::Other("unknown".to_owned())),
        ]
    }

    #[test]
    fn parse_empty_string_fails() {
        let err = "".parse::<Channel>().unwrap_err();

        assert_that!(err.current_context()).is_equal_to(ParseChannelError {
            value: String::new(),
        });
    }

    #[test]
    fn parse_channels() {
        for (test, expected) in channels() {
            assert_that!(test.parse::<Channel>())
                .is_ok()
                .is_equal_to(expected.clone());
        }
    }

    #[test]
    fn deserialize_channels() {
        for (test, expected) in channels() {
            assert_that!(serde_json::from_str::<Channel>(&format!(r#""{test}""#)))
                .is_ok()
                .is_equal_to(expected);
        }
    }

    #[test]
    fn serialized_channels() {
        fn capitalize_first(s: &str) -> String {
            s.chars()
                .take(1)
                .flat_map(|f| f.to_uppercase())
                .chain(s.chars().skip(1))
                .collect()
        }

        for (expected, channel) in channels() {
            assert_that!(serde_json::to_string(&channel))
                .is_ok()
                .is_equal_to(format!(
                    r#""{}""#,
                    match channel {
                        Channel::Other(_) => expected.to_owned(),
                        _ => capitalize_first(expected),
                    }
                ));
        }
    }
}
