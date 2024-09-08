//! # `vit_logger`
//!
//! Easy to use logger for your rust projects.
//!
//! ## Installation
//!
//! ```shell
//! cargo add vit-logger
//! ```
//!
//! ## Usage
//!
//! ```rust
//! use vit_logger::{Config, VitLogger};
//!
//! fn main() {
//!     std::env::set_var("RUST_LOG", "trace");
//!     VitLogger::new().init(Config::builder().finish()?);
//!     log::info!("Hello, world!");
//! }
//! ```

#![cfg_attr(test, deny(warnings))]

mod config;
mod formatter;

pub use config::{Config, ConfigBuilder};

extern crate log;

use crate::formatter::Formatter;
use log::{LevelFilter, Metadata, Record, SetLoggerError};

fn get_var() -> LevelFilter {
    std::env::var("RUST_LOG")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(LevelFilter::Info)
}

/// Implements the log::Log trait from `log` crate.
pub struct Logger {
    config: Config,
}

impl Logger {
    pub fn new(config: Config) -> Logger {
        Logger { config }
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= get_var()
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            Formatter::new(record, self.config).format();
        }
    }

    fn flush(&self) {}
}

/// The main struct for the logger.
#[derive(Default)]
pub struct VitLogger;
impl VitLogger {
    pub fn new() -> VitLogger {
        Default::default()
    }

    /// Initializes the logger with the given configuration.
    ///
    /// Should be called at the beginning of the program.
    /// Logs called before init will be ignored.
    pub fn try_init(&mut self, config: Config) -> Result<(), SetLoggerError> {
        let logger = Logger { config };

        let max_level: LevelFilter = get_var();
        let boxed = log::set_boxed_logger(Box::new(logger));

        if boxed.is_ok() {
            log::set_max_level(max_level);
        }

        boxed
    }

    /// Initializes the logger with the given configuration.
    ///
    /// Should be called at the beginning of the program.
    /// Logs called before init will be ignored.
    ///
    /// # Panics
    /// When the logger is called before init or another logger already was initialized.
    pub fn init(&mut self, config: Config) {
        self.try_init(config)
            .expect("Logger should not be called before init");
    }
}
