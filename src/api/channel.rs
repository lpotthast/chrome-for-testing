use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};
use std::str::FromStr;

/// Error returned when parsing a channel string fails.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error(
    "Unknown channel: '{value}'. Expected one of: Stable, Beta, Dev, Canary (or lowercased alternative)"
)]
pub struct ParseChannelError {
    value: String,
}

/// Chrome release channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
}

impl Display for Channel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Use the Debug implementation.
        write!(f, "{self:?}")
    }
}

impl FromStr for Channel {
    type Err = ParseChannelError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Stable" | "stable" => Ok(Channel::Stable),
            "Beta" | "beta" => Ok(Channel::Beta),
            "Dev" | "dev" => Ok(Channel::Dev),
            "Canary" | "canary" => Ok(Channel::Canary),
            _ => Err(ParseChannelError {
                value: s.to_owned(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assertr::prelude::*;

    #[test]
    fn parse_to_string_round_trip() {
        fn capitalize_first(s: &str) -> String {
            s.chars()
                .take(1)
                .flat_map(|f| f.to_uppercase())
                .chain(s.chars().skip(1))
                .collect()
        }

        let channels = [
            ("Stable", Channel::Stable),
            ("stable", Channel::Stable),
            ("Beta", Channel::Beta),
            ("beta", Channel::Beta),
            ("Dev", Channel::Dev),
            ("dev", Channel::Dev),
            ("Canary", Channel::Canary),
            ("canary", Channel::Canary),
        ];
        for (test, expected) in channels {
            assert_that!(test.parse::<Channel>())
                .is_ok()
                .is_equal_to(expected);
            assert_that!(expected.to_string()).is_equal_to(capitalize_first(test));
        }
    }

    #[test]
    fn parse_unknown_variants_failed() {
        assert_that!("unknown".parse::<Channel>())
            .is_err()
            .is_equal_to(ParseChannelError {
                value: "unknown".to_string(),
            });
    }

    #[test]
    fn serialized_value_matches_display_output() {
        assert_that!(serde_json::to_string(&Channel::Stable).unwrap())
            .is_equal_to(String::from("\"Stable\""));
    }
}
