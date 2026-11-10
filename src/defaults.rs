#![allow(dead_code)]

use log::LevelFilter;
use env_logger::TimestampPrecision;

use crate::logger::LogTarget;

// Build Constants:
pub(crate) const APP_NAME:   &str = "Dress";
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
pub(crate) const DEFAULT_LOG_FILE:             &'static str                                = "dress.log";
pub(crate) const DEFAULT_LOG_TARGET:           LogTarget                                   = LogTarget::Stdout;
//pub(crate) const DEFAULT_LOG_TARGET:           LogTarget                                   = LogTarget::File(DEFAULT_LOG_DIR, DEFAULT_LOG_FILE);

// Dress Defaults:
pub(crate) const DEFAULT_MODE:        &str = "cli";
pub(crate) const MODE_LIST:           &[&str] = &[ "cli","web","help"];
pub(crate) const DEFAULT_LLM_SERVER:  &str = "http://localhost:11434/api/generate";
pub(crate) const DEFAULT_MODEL:       &str = "gemma3:4b-it-qat";
pub(crate) const DEFAULT_DELAY:       u64  = 60;
pub(crate) const DEFAULT_LISTEN_ADDR: &str = "0.0.0.0:13874";

// Includes:
pub(crate) const SPLASH_PNG:             &[u8] = include_bytes!("../resources/dress.png");
pub(crate) const SPLASH_TXT:             &str  = include_str!("../resources/dress.txt");
pub(crate) const DRESS_SMALL_PNG:        &[u8] = include_bytes!("../resources/dress-small.png");
pub(crate) const WATERMARK_PNG:          &[u8] = include_bytes!("../resources/watermark-small.png");
pub(crate) const FAVICON_ICO:            &[u8] = include_bytes!("../resources/favicon.ico");
pub(crate) const BANNER_PNG:             &[u8] = include_bytes!("../resources/banner-small.png");
pub(crate) const FRAME_PNG:              &[u8] = include_bytes!("../resources/frame.png");
pub(crate) const HELP_TEMPLATE:          &str  = include_str!("../resources/help.tpl");
pub(crate) const SYSTEM_PROMPT_TEMPLATE: &str  = include_str!("../resources/system_prompt.tpl");
pub(crate) const USER_PROMPT_TEMPLATE:   &str  = include_str!("../resources/user_prompt.tpl");
pub(crate) const CSS_TEMPLATE:           &str = include_str!("../resources/css.tpl");
pub(crate) const INDEX_TEMPLATE:         &str = include_str!("../resources/index.tpl");
pub(crate) const TABS_TEMPLATE:          &str = include_str!("../resources/tabs.tpl");
pub(crate) const TEXT_ENTRY_TEMPLATE:    &str = include_str!("../resources/text_entry.tpl");
pub(crate) const DISPLAY_TEMPLATE:       &str = include_str!("../resources/display.tpl");

