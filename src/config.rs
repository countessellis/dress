use std::{
  collections::HashMap,
  env::{args,Args},
  fmt,
};

use crate::util;

use crate::defaults::*;

///////////// Config

#[derive(Debug, Clone)]
pub struct Config {
  pub config: HashMap<String,String>,
}

impl fmt::Display for Config {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let key_width: usize = self.config.keys().map(|k| k.len()).max().unwrap_or(0);
    let config: String = self.config.iter().map(|(parameter,value)| { let key = format!("    {}:",parameter); format!("{:<width$} {}",key,value, width=key_width) }).collect::<Vec<String>>().join("\n");
    write!(f,
"
  -----------------------------------
              Configuration
  -----------------------------------

{}

  -----------------------------------
",
      config,
    )
  }
}

impl Config {
  pub fn new() -> Config {
    let mut config: Self = Self {
      config: HashMap::new(),
    };
    config.from_defaults();
    config.from_args();
    config
  }

  pub fn get(&self, parameter: &str) -> String {
    self.config.get(&parameter.to_string()).cloned().unwrap_or_default()
  }

  pub fn set(&mut self, parameter: &String, value: &String) -> Option<String> {
    self.config.insert(parameter.clone(),value.clone())
  }

  pub fn from_defaults(&mut self) {
    log::debug!("Loading config with defaults.");
    self.config = HashMap::from([
      ("ranas".to_string(),util::bin_name()),
      ("llm_server".to_string(),DEFAULT_LLM_SERVER.to_string()),
      ("model".to_string(),DEFAULT_MODEL.to_string()),
      ("listen".to_string(),DEFAULT_LISTEN_ADDR.to_string()),
    ]);
  }

  pub fn to_string(&self) -> String {
    log::debug!("Converting config to string.");
    self.config.iter().map(|(parameter,value)| format!("{}: {}",parameter,value)).collect::<Vec<String>>().join("\n")
  }

  pub fn from_string(&mut self, source: &String) {
    log::debug!("Reading config from provided string.");
    for line in source.split("\n") {
      let pair: Vec<&str> = line.split(":").collect();
      if pair.len() > 1 {
        let value: String = pair[1..].join(":").trim_start().to_string();
        log::debug!("Saving parameter '{}' with value '{}' to config.",pair[0],value);
        self.config.insert(pair[0].to_string(),value);
      }
    }
  }

  pub fn from_args(&mut self) {
    log::debug!("Reading config from arguments.");
    let config = &mut self.config;
    let mut args: Args = args();
    while let Some(arg) = args.next() {
      match arg.as_str() {
        option => {
          if option.starts_with("--") {
            if let Some(option) = option.strip_prefix("--") {
              match args.next() {
                Some(value) => { config.insert(option.to_string(),value.to_string()); },
                None => {},
              }
            }
          }
        },
      }
    }
  }
}

