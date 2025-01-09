use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer};
use std::cmp::PartialOrd;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
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

// Thanks chatgpt
impl<'de> Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct VersionVisitor;

        impl Visitor<'_> for VersionVisitor {
            type Value = Version;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("a version string in dot format")
            }

            fn visit_str<E>(self, value: &str) -> Result<Version, E>
            where
                E: de::Error,
            {
                let parts: Vec<&str> = value.split('.').collect();

                if parts.len() == 4 {
                    let major: Result<u32, E> = parts[0].parse().map_err(de::Error::custom);
                    let minor: Result<u32, E> = parts[1].parse().map_err(de::Error::custom);
                    let patch: Result<u32, E> = parts[2].parse().map_err(de::Error::custom);
                    let build: Result<u32, E> = parts[3].parse().map_err(de::Error::custom);

                    Ok(Version {
                        major: major?,
                        minor: minor?,
                        patch: patch?,
                        build: build?,
                    })
                } else {
                    Err(de::Error::custom("Invalid version string format"))
                }
            }
        }

        deserializer.deserialize_str(VersionVisitor)
    }
}
