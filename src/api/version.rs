use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// Error returned when parsing a version string fails.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("{message}")]
pub struct ParseVersionError {
    message: String,
}

/// A version identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Version {
    /// The major version number.
    pub major: u32,

    /// The minor version number.
    pub minor: u32,

    /// The patch version number.
    pub patch: u32,

    /// The build version number.
    pub build: u32,
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}.{}.{}.{}",
            self.major, self.minor, self.patch, self.build
        ))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        self.major
            .cmp(&other.major)
            .then(self.minor.cmp(&other.minor))
            .then(self.patch.cmp(&other.patch))
            .then(self.build.cmp(&other.build))
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_version(value: &str) -> Result<Version, String> {
    fn parse_part<'i>(
        parts: &mut impl Iterator<Item = &'i str>,
        named: &'static str,
    ) -> Result<u32, String> {
        parts
            .next()
            .ok_or_else(|| format!("Did not find part '{named}'."))?
            .parse::<u32>()
            .map_err(|err| format!("Failed to parse '{named}' part as an u32: {err}"))
    }

    let mut parts = value.split('.');
    let major = parse_part(&mut parts, "major")?;
    let minor = parse_part(&mut parts, "minor")?;
    let patch = parse_part(&mut parts, "patch")?;
    let build = parse_part(&mut parts, "build")?;

    if let Some(next) = parts.next() {
        return Err(format!(
            "Invalid version string format. Did not expect any additional parts. Got at least the additional part: {next}"
        ));
    }

    Ok(Version {
        major,
        minor,
        patch,
        build,
    })
}

impl FromStr for Version {
    type Err = ParseVersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_version(s).map_err(|message| ParseVersionError { message })
    }
}

impl Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct VersionVisitor;

        impl Visitor<'_> for VersionVisitor {
            type Value = Version;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("a version string in dot format \"{major}.{minor}.{patch}.{build}\", like `1.0.0.0`, with each part being a `u32`, and all parts being required")
            }

            fn visit_str<E>(self, value: &str) -> Result<Version, E>
            where
                E: de::Error,
            {
                parse_version(value).map_err(de::Error::custom)
            }
        }

        deserializer.deserialize_str(VersionVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assertr::prelude::*;
    use std::collections::HashSet;

    #[test]
    fn deserialize_valid_version() {
        let test_cases = vec![
            (
                "0.0.0.0",
                Version {
                    major: 0,
                    minor: 0,
                    patch: 0,
                    build: 0,
                },
            ),
            (
                "1.2.3.4",
                Version {
                    major: 1,
                    minor: 2,
                    patch: 3,
                    build: 4,
                },
            ),
            (
                "115.785.5763.42",
                Version {
                    major: 115,
                    minor: 785,
                    patch: 5763,
                    build: 42,
                },
            ),
            (
                "4294967295.4294967295.4294967295.4294967295",
                Version {
                    major: u32::MAX,
                    minor: u32::MAX,
                    patch: u32::MAX,
                    build: u32::MAX,
                },
            ),
        ];

        for (input, expected) in test_cases {
            let json = format!(
                r#"
                "{input}"
            "#
            );
            let result = serde_json::from_str::<Version>(&json);
            assert_that!(result).is_ok().is_equal_to(expected);
        }
    }

    #[test]
    fn deserialize_invalid_version() {
        let invalid_cases = vec![
            ("", "Failed to parse 'major' part as an u32"),
            ("1", "Did not find part 'minor'"),
            ("1.2", "Did not find part 'patch'"),
            ("1.2.3", "Did not find part 'build'"),
            ("1.2.3.4.5", "Did not expect any additional parts"),
            ("a.b.c.d", "Failed to parse 'major' part as an u32"),
            ("1.b.c.d", "Failed to parse 'minor' part as an u32"),
            ("1.2.c.d", "Failed to parse 'patch' part as an u32"),
            ("1.2.3.d", "Failed to parse 'build' part as an u32"),
            ("-1.2.3.4", "Failed to parse 'major' part as an u32"),
            ("1.-2.3.4", "Failed to parse 'minor' part as an u32"),
            ("1.2.-3.4", "Failed to parse 'patch' part as an u32"),
            ("1.2.3.-4", "Failed to parse 'build' part as an u32"),
            ("4294967296.0.0.0", "Failed to parse 'major' part as an u32"),
            ("0.4294967296.0.0", "Failed to parse 'minor' part as an u32"),
            ("0.0.4294967296.0", "Failed to parse 'patch' part as an u32"),
            ("0.0.0.4294967296", "Failed to parse 'build' part as an u32"),
            (".2.3.4", "Failed to parse 'major' part as an u32"),
            ("1..3.4", "Failed to parse 'minor' part as an u32"),
            ("1.2..4", "Failed to parse 'patch' part as an u32"),
            ("1.2.3.", "Failed to parse 'build' part as an u32"),
        ];

        for (input, expected_error_substring) in invalid_cases {
            let json = format!(
                r#"
                "{input}"
            "#
            );
            let result = serde_json::from_str::<Version>(&json);
            assert_that!(result)
                .is_err()
                .derive(|it| it.to_string())
                .contains(expected_error_substring);
        }
    }

    #[test]
    fn display_value() {
        let version = Version {
            major: 115,
            minor: 785,
            patch: 5763,
            build: 42,
        };
        assert_that!(version).has_display_value("115.785.5763.42");
    }

    #[test]
    fn debug_value() {
        let version = Version {
            major: 115,
            minor: 785,
            patch: 5763,
            build: 42,
        };
        assert_that!(version)
            .has_debug_value("Version { major: 115, minor: 785, patch: 5763, build: 42 }");
    }

    #[test]
    fn comparisons() {
        let v1 = Version {
            major: 1,
            minor: 0,
            patch: 0,
            build: 0,
        };
        let v2 = Version {
            major: 1,
            minor: 0,
            patch: 0,
            build: 1,
        };
        let v3 = Version {
            major: 1,
            minor: 0,
            patch: 1,
            build: 0,
        };
        let v4 = Version {
            major: 1,
            minor: 1,
            patch: 0,
            build: 0,
        };
        let v5 = Version {
            major: 2,
            minor: 0,
            patch: 0,
            build: 0,
        };

        assert_that!(v1).is_less_than(v2);
        assert_that!(v1).is_less_than(v3);
        assert_that!(v1).is_less_than(v4);
        assert_that!(v1).is_less_than(v5);
        assert_that!(v2).is_less_than(v3);
        assert_that!(v2).is_less_than(v4);
        assert_that!(v2).is_less_than(v5);
        assert_that!(v3).is_less_than(v4);
        assert_that!(v3).is_less_than(v5);
        assert_that!(v4).is_less_than(v5);

        assert_that!(v2).is_greater_than(v1);
        assert_that!(v3).is_greater_than(v1);
        assert_that!(v4).is_greater_than(v1);
        assert_that!(v5).is_greater_than(v1);
        assert_that!(v3).is_greater_than(v2);
        assert_that!(v4).is_greater_than(v2);
        assert_that!(v5).is_greater_than(v2);
        assert_that!(v4).is_greater_than(v3);
        assert_that!(v5).is_greater_than(v3);
        assert_that!(v5).is_greater_than(v4);

        assert_that!(v1).is_equal_to(v1);
        assert_that!(v1).is_less_or_equal_to(v1);
        assert_that!(v1).is_greater_or_equal_to(v1);

        assert_that!(v2).is_equal_to(v2);
        assert_that!(v2).is_less_or_equal_to(v2);
        assert_that!(v2).is_greater_or_equal_to(v2);

        assert_that!(v3).is_equal_to(v3);
        assert_that!(v3).is_less_or_equal_to(v3);
        assert_that!(v3).is_greater_or_equal_to(v3);

        assert_that!(v4).is_equal_to(v4);
        assert_that!(v4).is_less_or_equal_to(v4);
        assert_that!(v4).is_greater_or_equal_to(v4);

        assert_that!(v5).is_equal_to(v5);
        assert_that!(v5).is_less_or_equal_to(v5);
        assert_that!(v5).is_greater_or_equal_to(v5);
    }

    #[test]
    fn parse_valid() {
        assert_that!("1.2.3.4".parse::<Version>())
            .is_ok()
            .is_equal_to(Version {
                major: 1,
                minor: 2,
                patch: 3,
                build: 4,
            });
        assert_that!("0.0.0.0".parse::<Version>())
            .is_ok()
            .is_equal_to(Version {
                major: 0,
                minor: 0,
                patch: 0,
                build: 0,
            });
    }

    #[test]
    fn parse_invalid_fails() {
        assert_that!("".parse::<Version>()).is_err();
        assert_that!("1".parse::<Version>()).is_err();
        assert_that!("1.2".parse::<Version>()).is_err();
        assert_that!("1.2.3".parse::<Version>()).is_err();
        assert_that!("1.2.3.4.5".parse::<Version>()).is_err();
        assert_that!("a.b.c.d".parse::<Version>()).is_err();
    }

    #[test]
    fn serialize_round_trip() {
        let version = Version {
            major: 132,
            minor: 0,
            patch: 6834,
            build: 83,
        };
        let json = serde_json::to_string(&version).unwrap();
        assert_that!(json.clone()).is_equal_to(String::from("\"132.0.6834.83\""));
        let deserialized: Version = serde_json::from_str(&json).unwrap();
        assert_that!(deserialized).is_equal_to(version);
    }

    #[test]
    fn ord_sorting() {
        let v3 = Version {
            major: 2,
            minor: 0,
            patch: 0,
            build: 0,
        };
        let v1 = Version {
            major: 1,
            minor: 0,
            patch: 0,
            build: 0,
        };
        let v2 = Version {
            major: 1,
            minor: 1,
            patch: 0,
            build: 0,
        };
        let mut versions = vec![v3, v1, v2];
        versions.sort();
        assert_that!(versions).is_equal_to(vec![v1, v2, v3]);
    }

    #[test]
    fn hash_in_set() {
        let v1 = Version {
            major: 1,
            minor: 0,
            patch: 0,
            build: 0,
        };
        let v2 = Version {
            major: 1,
            minor: 0,
            patch: 0,
            build: 0,
        };
        let mut set = HashSet::new();
        set.insert(v1);
        set.insert(v2);
        assert_that!(set.len()).is_equal_to(1);
    }
}
