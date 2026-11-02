#![allow(dead_code)]

use log::LevelFilter;
use env_logger::TimestampPrecision;

use crate::logger::LogTarget;

// Build Constants:
pub(crate) const APP_NAME:   &str = "Backless Dress";
pub(crate) const BUILD_NAME: &str = env!("BUILD_NAME");
pub(crate) const VERSION_ID: &str = env!("CARGO_PKG_VERSION");
pub(crate) const BUILD_TIME: &str = env!("BUILD_TIME");
pub(crate) const BUILD_ID:   &str = env!("BUILD_ID");
pub(crate) const AUTHORS:    &str = env!("CARGO_PKG_AUTHORS");
pub(crate) const COPYRIGHT:  &str = "2026";
pub(crate) const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
pub(crate) const LICENSE:    &str = env!("CARGO_PKG_LICENSE");

// Logging Constants:
pub(crate) const DEFAULT_LOG_LEVEL:            LevelFilter                                 = LevelFilter::Debug; // LevelFilter::Info;
pub(crate) const DEFAULT_LOG_FORMAT_TIMESTAMP: Option<env_logger::fmt::TimestampPrecision> = Some(TimestampPrecision::Millis); // None;
pub(crate) const DEFAULT_LOG_FORMAT_LEVEL:     bool                                        = true; // false;
pub(crate) const DEFAULT_LOG_FORMAT_TARGET:    bool                                        = true; // false;
pub(crate) const DEFAULT_LOG_DIR:              &'static str                                = "logs";
pub(crate) const DEFAULT_LOG_FILE:             &'static str                                = "backless-dress.log";
pub(crate) const DEFAULT_LOG_TARGET:           LogTarget                                   = LogTarget::Stdout;
//pub(crate) const DEFAULT_LOG_TARGET:           LogTarget                                   = LogTarget::File(DEFAULT_LOG_DIR, DEFAULT_LOG_FILE);

// Backless Dress Defaults:
pub(crate) const DEFAULT_MODE: &str = "CLI";
pub(crate) const MODE_LIST:    &[&str] = &[ "cli","tui","gui","web","help"];


// Includes:
pub(crate) const SPLASH_PNG:    &[u8] = include_bytes!("../resources/backless-dress.png");
pub(crate) const SPLASH_TXT:    &str  = include_str!("../resources/backless-dress.txt");
pub(crate) const HELP_TEMPLATE: &str  = include_str!("../resources/help.tpl");
