use lazy_static::lazy_static;
use log::{LevelFilter, Record};
use std::collections::HashMap;
use std::path::Path;

extern crate chrono;

use crate::config::Config;
use chrono::Local;
use colored::Colorize;

lazy_static! {
    pub static ref TYPE_ICONS: HashMap<LevelFilter, &'static str> = {
        let mut m = HashMap::new();
        m.insert(LevelFilter::Error, "\u{2717}");
        m.insert(LevelFilter::Warn, "\u{25EC}");
        m.insert(LevelFilter::Info, "\u{2756}");
        m.insert(LevelFilter::Debug, "\u{2713}");
        m.insert(LevelFilter::Trace, "\u{25C9}");
        m
    };
}

pub struct Formatter {
    level: LevelFilter,
    message: String,
    config: Config,
    file: Option<String>,
    line: Option<u32>,
}

impl<'a> Formatter {
    pub fn new(record: &Record, config: Config) -> Formatter {
        Formatter {
            level: record.level().to_level_filter(),
            message: record.args().to_string(),
            config,
            file: record.file().map(|f| f.to_string()),
            line: record.line(),
        }
    }
    pub fn format(&self) {
        let target = if self.config.target {
            format!("{}{} ", env!("CARGO_PKG_NAME").dimmed(), ":".dimmed())
        } else {
            "".to_string()
        };

        let time = self.get_local_time().dimmed();
        let left = if self.config.badge {
            format!("{} {target}{}", self.get_badge(), self.message)
        } else if self.config.icon {
            format!(
                "{} {target}{}",
                self.colorize(self.get_icon(self.level)),
                self.message
            )
        } else if self.config.text {
            format!("{} {target}{}", self.colorize(&self.level.to_string().to_lowercase()), self.message)
        } else {
            format!(
                "{} {target}{}",
                self.colorize(self.get_icon(self.level)),
                self.message
            )
        };

        let normalized_path = &self.file.as_deref().unwrap_or("").replace("\\", "/");
        let line = if self.line.is_some() {
            format!("{}{}", ":".dimmed(), self.line.unwrap().to_string().dimmed())
        } else {
            "".to_string()
        };

        let location = if self.config.file && self.config.line {
            format!(
                "{}{}",
                normalized_path.dimmed(),
                line
            )
        } else if self.config.file {
            format!("{}", normalized_path.dimmed())
        } else if self.config.line {
            format!("{}", line)
        } else {
            "".to_string()
        };

        let line = format!("{}{}{}", if self.config.time {
            format!(" {} ", time)
        } else {
            "".to_string()
        }, if location.is_empty() {
            "".to_string()
        } else {
            format!("{} ", location)
        }, left);

        if self.config.badge {
            println!("\n{}\n", line);
        } else {
            println!("{}", line);
        }
    }

    pub fn get_icon(&self, level: LevelFilter) -> &str {
        TYPE_ICONS.get(&level).unwrap_or(&"")
    }

    pub fn colorize(&self, msg: &str) -> colored::ColoredString {
        match self.level {
            LevelFilter::Off => msg.red(),
            LevelFilter::Error => msg.bright_red(),
            LevelFilter::Warn => msg.yellow(),
            LevelFilter::Info => msg.bright_cyan(),
            LevelFilter::Debug => msg.green(),
            LevelFilter::Trace => msg.bright_magenta(),
        }
    }

    pub fn colorize_badge(&self, msg: &str) -> colored::ColoredString {
        match self.level {
            LevelFilter::Off => msg.on_red(),
            LevelFilter::Error => msg.on_bright_red(),
            LevelFilter::Warn => msg.on_yellow(),
            LevelFilter::Info => msg.on_bright_cyan(),
            LevelFilter::Debug => msg.on_green(),
            LevelFilter::Trace => msg.on_bright_magenta(),
        }
    }

    pub fn get_local_time(&self) -> String {
        let date = Local::now();
        date.format("%H:%M:%S").to_string()
    }

    pub fn get_badge(&self) -> String {
        let level_name = match self.level {
            LevelFilter::Error => " error ",
            LevelFilter::Warn => " warn ",
            LevelFilter::Info => " info ",
            LevelFilter::Debug => " debug ",
            LevelFilter::Trace => " trace ",
            _ => "off",
        };

        self.colorize_badge(&level_name.to_uppercase())
            .black()
            .to_string()
    }
}