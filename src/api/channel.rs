use serde::Deserialize;
use std::fmt::{Debug, Display};

/// Chrome release channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use assertr::prelude::*;

    #[test]
    fn display_impl_formats_channels_as_expected() {
        assert_that(format!("{}", Channel::Stable)).is_equal_to("Stable");
        assert_that(format!("{}", Channel::Beta)).is_equal_to("Beta");
        assert_that(format!("{}", Channel::Dev)).is_equal_to("Dev");
        assert_that(format!("{}", Channel::Canary)).is_equal_to("Canary");
    }
}
