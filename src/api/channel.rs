use serde::Deserialize;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
pub enum Channel {
    Stable,
    Beta,
    Dev,
    Canary,
}

impl Display for Channel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assertr::prelude::*;

    #[test]
    fn display_impl_formats_channels_as_expected(){
        assert_that(format!("{}", Channel::Stable)).is_equal_to("Stable");
        assert_that(format!("{}", Channel::Beta)).is_equal_to("Beta");
        assert_that(format!("{}", Channel::Dev)).is_equal_to("Dev");
        assert_that(format!("{}", Channel::Canary)).is_equal_to("Canary");
    }
}