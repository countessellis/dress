use std::fmt;
use std::collections::HashMap;

use crate::util;

use crate::collections::{Config,Fields};

use crate::defaults::*;

///////////// Help

#[derive(Debug,Clone)]
pub(crate) struct Help {
  config: Config,
}

impl fmt::Display for Help {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let details: Vec<String> = vec![
      format!("  --mode <MODE>                        Mode to run in (default: {}, choices: {})",DEFAULT_MODE,MODE_LIST.join(", ")),
      format!(""),
      format!("  --llm_server                         LLM server to interpret  the text, ollama compatible, (default: {})",DEFAULT_LLM_SERVER),
      format!("  --model                              LLM model to use to interpret the text (default: {})",DEFAULT_MODEL),
      format!("  --listen                             In web mode, address and port to listen on (default: {})",DEFAULT_LISTEN_ADDR),
      format!("  --source                             The text source, which can be a URL or a file path, no default."),
      format!(""),
      format!("  --help                               Display this help message"),
      format!("  --version                            Display the version"),
    ];
    let fields: Fields = Fields::new(&HashMap::from([
      ("RUNAS",self.config.get("ranas")),
      ("DESCRIPTION",DESCRIPTION.to_string()),
      ("DETAILS",details.join("\n")),
      ("COPYRIGHT",COPYRIGHT.to_string()),
      ("AUTHORS",AUTHORS.to_string()),
    ]));
    let help: String = util::render(&HELP_TEMPLATE.to_string(),&fields);
    write!(f, "{}",help)
  }
}

impl Help {
  pub(crate) fn new(config: &Config) -> Help {
    Help { config: config.clone() }
  }
}

