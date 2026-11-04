use log::{LevelFilter,Level};
use std::str::FromStr;
use std::io::Write;
use env_logger::TimestampPrecision;
use std::fs::create_dir_all;
use std::path::PathBuf;
use std::fs::OpenOptions;

use crate::defaults::*;

///////////// Logger

#[allow(dead_code)]
pub(crate) enum LogTarget {
  Stdout,
  File(&'static str,&'static str),
}

impl LogTarget {
  fn into_env_logger_target(self) -> env_logger::fmt::Target {
    match self {
      LogTarget::Stdout => env_logger::fmt::Target::Stdout,
      LogTarget::File(dir, filename) => {
        if !dir.is_empty() {
          if let Err(err) = create_dir_all(dir) {
            eprintln!("Warning: Failed to create log directory '{}' ({}). Falling back to Stdout.", dir, err);
            return env_logger::fmt::Target::Stdout;
          }
        }
        let mut path = PathBuf::from(dir);
        path.push(filename);
        match OpenOptions::new().create(true).append(true).open(&path) {
          Ok(file) => env_logger::fmt::Target::Pipe(Box::new(file)),
          Err(err) => {
            eprintln!("Warning: Failed to open log file '{:?}' ({}). Falling back to Stdout.", path, err);
            env_logger::fmt::Target::Stdout
          }
        }
      },
    }
  }
}

pub fn init() {
  let loglevel: LevelFilter = match std::env::var("RUST_LOG") {
    Ok(level) => match Level::from_str(level.as_str()) {
      Ok(level) => level.to_level_filter(),
      Err(_)    => DEFAULT_LOG_LEVEL,
    },
    Err(_)    => DEFAULT_LOG_LEVEL,
  };

  env_logger::Builder::from_default_env()
    .filter_level(loglevel)
    .filter_module("ureq",log::LevelFilter::Info)
    .filter_module("html5ever::tree_builder",log::LevelFilter::Info)
    .filter_module("html5ever::tokenizer",log::LevelFilter::Info)
    .target(DEFAULT_LOG_TARGET.into_env_logger_target())
    .format(move |buf, record| {
      let msg = format!("{}", record.args());
      let mut metadata = Vec::new();
      if let Some(precision) = DEFAULT_LOG_FORMAT_TIMESTAMP {
        let ts = match precision {
          TimestampPrecision::Seconds => format!("{}", buf.timestamp_seconds()),
          TimestampPrecision::Millis  => format!("{}", buf.timestamp_millis()),
          TimestampPrecision::Micros  => format!("{}", buf.timestamp_micros()),
          TimestampPrecision::Nanos   => format!("{}", buf.timestamp_nanos()),
        };
        metadata.push(ts);
      }
      if DEFAULT_LOG_FORMAT_LEVEL {
        metadata.push(format!("{}", record.level()));
      }
      if DEFAULT_LOG_FORMAT_TARGET {
        metadata.push(format!("{}", record.target()));
      }
      if metadata.is_empty() {
        writeln!(buf, "[] {}", msg)
      } else {
        writeln!(buf, "[{}] {}", metadata.join(" "), msg)
      }
    })
    .init();
}

