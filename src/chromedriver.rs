use std::fmt::{Display, Formatter};

/// The chromedriver binary can be passed one of these log-levels.
///
/// Defaults to `Info`. This is enough to get the "started on port ..." line on stdout.
///
/// For example, if launching chromedriver with std/tokio `Process`, do this:
/// ```no_run
/// let mut command = std::process::Command::new("chromedriver");
/// let loglevel = chrome_for_testing::chromedriver::LogLevel::All;
/// command.arg(format!("--log-level={loglevel}"));
/// ```
#[derive(Default, Debug, PartialEq, Eq)]
pub enum LogLevel {
    All,
    Debug,
    #[default]
    Info,
    Warning,
    Severe,
    Off,
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            LogLevel::All => "ALL",
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warning => "WARNING",
            LogLevel::Severe => "SEVERE",
            LogLevel::Off => "OFF",
        })
    }
}
