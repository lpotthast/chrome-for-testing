use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer};
use std::cmp::PartialOrd;
use std::fmt::{Display, Formatter};

/// A version identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.major != other.major {
            return self.major.partial_cmp(&other.major);
        }
        if self.minor != other.minor {
            return self.minor.partial_cmp(&other.minor);
        }
        if self.patch != other.patch {
            return self.patch.partial_cmp(&other.patch);
        }
        if self.build != other.build {
            return self.build.partial_cmp(&other.build);
        }
        Some(std::cmp::Ordering::Equal)
    }

    fn lt(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(std::cmp::Ordering::Less)
    }

    fn le(&self, other: &Self) -> bool {
        self.partial_cmp(other) != Some(std::cmp::Ordering::Greater)
    }

    fn gt(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(std::cmp::Ordering::Greater)
    }

    fn ge(&self, other: &Self) -> bool {
        self.partial_cmp(other) != Some(std::cmp::Ordering::Less)
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
                let mut parts = value.split('.');

                fn parse_part<'i, E: de::Error>(
                    parts: &mut impl Iterator<Item = &'i str>,
                    named: &'static str,
                ) -> Result<u32, E> {
                    parts
                        .next()
                        .ok_or_else(|| de::Error::custom(format!("Did not find part '{named}'.")))?
                        .parse::<u32>()
                        .map_err(|err| {
                            de::Error::custom(format!(
                                "Failed to parse '{named}' part as an u32: {err}"
                            ))
                        })
                }

                let major = parse_part(&mut parts, "major")?;
                let minor = parse_part(&mut parts, "minor")?;
                let patch = parse_part(&mut parts, "patch")?;
                let build = parse_part(&mut parts, "build")?;

                if let Some(next) = parts.next() {
                    return Err(de::Error::custom(format!(
                        "Invalid version string format. Did not expect any additional parts. Got at least the additional part: {next}"
                    )));
                }

                Ok(Version {
                    major,
                    minor,
                    patch,
                    build,
                })
            }
        }

        deserializer.deserialize_str(VersionVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assertr::prelude::*;

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
            assert_that(result).is_ok().is_equal_to(expected);
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
            assert_that(result)
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
        assert_that(version).has_display_value("115.785.5763.42");
    }

    #[test]
    fn debug_value() {
        let version = Version {
            major: 115,
            minor: 785,
            patch: 5763,
            build: 42,
        };
        assert_that(version)
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

        assert_that(v1).is_less_than(v2);
        assert_that(v1).is_less_than(v3);
        assert_that(v1).is_less_than(v4);
        assert_that(v1).is_less_than(v5);
        assert_that(v2).is_less_than(v3);
        assert_that(v2).is_less_than(v4);
        assert_that(v2).is_less_than(v5);
        assert_that(v3).is_less_than(v4);
        assert_that(v3).is_less_than(v5);
        assert_that(v4).is_less_than(v5);

        assert_that(v2).is_greater_than(v1);
        assert_that(v3).is_greater_than(v1);
        assert_that(v4).is_greater_than(v1);
        assert_that(v5).is_greater_than(v1);
        assert_that(v3).is_greater_than(v2);
        assert_that(v4).is_greater_than(v2);
        assert_that(v5).is_greater_than(v2);
        assert_that(v4).is_greater_than(v3);
        assert_that(v5).is_greater_than(v3);
        assert_that(v5).is_greater_than(v4);

        assert_that(v1).is_equal_to(v1);
        assert_that(v1).is_less_or_equal_to(v1);
        assert_that(v1).is_greater_or_equal_to(v1);

        assert_that(v2).is_equal_to(v2);
        assert_that(v2).is_less_or_equal_to(v2);
        assert_that(v2).is_greater_or_equal_to(v2);

        assert_that(v3).is_equal_to(v3);
        assert_that(v3).is_less_or_equal_to(v3);
        assert_that(v3).is_greater_or_equal_to(v3);

        assert_that(v4).is_equal_to(v4);
        assert_that(v4).is_less_or_equal_to(v4);
        assert_that(v4).is_greater_or_equal_to(v4);

        assert_that(v5).is_equal_to(v5);
        assert_that(v5).is_less_or_equal_to(v5);
        assert_that(v5).is_greater_or_equal_to(v5);
    }
}
