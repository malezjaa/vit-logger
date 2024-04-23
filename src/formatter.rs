use colorized::{Color, Colors};
use lazy_static::lazy_static;
use log::{LevelFilter, Record};
use std::collections::HashMap;
extern crate chrono;

use crate::config::Config;
use chrono::Local;

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
}

impl<'a> Formatter {
    pub fn new(record: &Record, config: Config) -> Formatter {
        Formatter {
            level: record.level().to_level_filter(),
            message: record.args().to_string(),
            config,
        }
    }
    pub fn format(&self) {
        let right = self.get_local_time().color(Colors::BlackFg);
        let left = if self.config.badge {
            format!("{} {}", self.get_badge(), self.message)
        } else {
            format!(
                "{} {}",
                self.colorize(self.get_icon(self.level)),
                self.message
            )
        };

        let line = format!("{} {}", right, left);

        if self.config.badge {
            println!("\n{}\n", line);
        } else {
            println!("{}", line);
        }
    }

    pub fn get_icon(&self, level: LevelFilter) -> &str {
        TYPE_ICONS.get(&level).unwrap_or(&"")
    }

    pub fn colorize(&self, msg: &str) -> String {
        match self.level {
            LevelFilter::Off => msg.color(Colors::RedFg),
            LevelFilter::Error => msg.color(Colors::RedFg),
            LevelFilter::Warn => msg.color(Colors::YellowFg),
            LevelFilter::Info => msg.color(Colors::CyanFg),
            LevelFilter::Debug => msg.color(Colors::GreenFg),
            LevelFilter::Trace => msg.color(Colors::MagentaFg),
        }
    }

    pub fn colorize_badge(&self, msg: &str) -> String {
        match self.level {
            LevelFilter::Off => msg.color(Colors::RedBg),
            LevelFilter::Error => msg.color(Colors::RedBg),
            LevelFilter::Warn => msg.color(Colors::YellowBg),
            LevelFilter::Info => msg.color(Colors::CyanBg),
            LevelFilter::Debug => msg.color(Colors::GreenBg),
            LevelFilter::Trace => msg.color(Colors::MagentaBg),
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
            .color(Colors::BlackFg)
            .to_string()
    }
}
