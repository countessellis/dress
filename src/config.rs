use std::collections::HashMap;

use crate::util;

use crate::collections::{Config,Configurator};

use crate::defaults::*;

///////////// Config

pub(crate) fn init() -> Config {
  let configurator = Configurator {
    defaults: HashMap::from([
      ("ranas".to_string(),util::bin_name()),
      ("llm_server".to_string(),DEFAULT_LLM_SERVER.to_string()),
      ("model".to_string(),DEFAULT_MODEL.to_string()),
      ("listen".to_string(),DEFAULT_LISTEN_ADDR.to_string()),
    ]),
    file: None,
    ignore: vec![
      "--cli".to_string(),
      "--web".to_string(),
      "--help".to_string(),
    ],
    flags: HashMap::new(),
  };
  configurator.to_config()
}
