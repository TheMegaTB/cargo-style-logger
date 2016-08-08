extern crate log;
extern crate ansi_term;

use log::{LogRecord, LogLevel, LogMetadata, set_logger};
use std::env;
use std::str::FromStr;
pub use ansi_term::*;

use std::error::Error;

/// The logger type responsible for printing that sexy output you see when launching BitDMX
pub struct Logger {
    level: LogLevel,
    show_paths: bool
}

impl Logger {
    /// This function initializes the logger and enables it.
    ///
    /// When enabling it reads the following environment variables for configuration:
    ///
    /// `LOG`      the loglevel at which it may print (trace, debug, info, warn, error)
    ///
    /// `PATHS`    whether or not to show the origin of a message (true, false)
    pub fn init(default_level: LogLevel) {
        match set_logger(|max_log_level| {
            let level = match env::var("LOG") {
                Ok(level) => {
                    match LogLevel::from_str(&level) {
                        Ok(level) => level,
                        Err(_) => default_level
                    }
                },
                Err(_) => default_level
            };
            let show_paths = match env::var("PATHS") {
                Ok(val) => val == String::from("true"),
                Err(_) => false
            };
            max_log_level.set(level.to_log_level_filter());
            Box::new(Logger {
                level: level,
                show_paths: show_paths
            })
        }) {
            Ok(_) => {},
            Err(e) => {
                println!("{} Failed to set logger: {}", Colour::Fixed(160).bold().paint("       Error"), e.description());
                ::std::process::exit(6);
            }
        }
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            let path = match self.level {
                LogLevel::Trace => {
                    let loc = record.location();
                    format!("{}:{}", loc.file(), loc.line())
                },
                LogLevel::Debug => {
                    format!("{}", record.location().module_path())
                },
                _ => {String::new()}
            };
            let level = match record.level() {
                LogLevel::Error => { Colour::Fixed(160).bold().paint("       Error") },
                LogLevel::Warn  => { Colour::Fixed(214).bold().paint("     Warning") },
                LogLevel::Info  => { Colour::Fixed( 10).bold().paint("        Info") },
                LogLevel::Debug => { Colour::Fixed(244).bold().paint("       Debug") },
                LogLevel::Trace => { Colour::Fixed(239).bold().paint("       Trace") },
            };
            if self.show_paths {
                println!("{} {}\n             {}", level, record.args(), Colour::Fixed(239).paint(path));
            } else {
                println!("{} {}", level, record.args());
            }
        }
    }
}

/// Panic with a given error code and print an optional message
/// # Examples
///
/// ```should_panic
/// # #[macro_use] extern crate structures;
/// # #[macro_use] extern crate log;
/// # fn main() {
/// // An error code is required
/// exit!(1);
/// # }
/// ```
///
/// ```should_panic
/// # #[macro_use] extern crate structures;
/// # #[macro_use] extern crate log;
/// # fn main() {
/// // Additionally you can provide an error message
/// exit!(1, "Some random generic error.");
/// # }
/// ```
///
/// ```should_panic
/// # #[macro_use] extern crate structures;
/// # #[macro_use] extern crate log;
/// # fn main() {
/// // It's even possible to use format arguments
/// exit!(1, "Some random generic error. And some nice arguments are possible as well: {}", 5);
/// # }
/// ```
#[macro_export]
macro_rules! exit {
    () => {exit!(1)};
    ($code:expr) => {
        // TODO Save all that important work
        ::std::process::exit($code);
    };
    ($code:expr, $res:expr) => {
        error!("{}", $res);
        exit!($code);
    };
    ($code:expr, $res:expr, $($arg:tt)*) => {
        exit!($code, format!($res, $($arg)*));
    };
}
