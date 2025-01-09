use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
pub enum Channel {
    Stable,
    Beta,
    Dev,
    Canary,
}
