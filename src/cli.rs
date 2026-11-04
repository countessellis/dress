use crate::config::*;
use crate::llm::*;
use crate::source::*;

///////////// CLI

#[derive(Clone)]
pub(crate) struct CLI {
  pub(crate) config: Config,
  pub(crate) source: Option<Source>,
  pub(crate) llm:    LLM,
}

impl CLI {
  pub(crate) fn new(config: &Config) -> CLI {
    CLI {
     config: config.clone(),
     source: Source::load(&config.get("source")),
     llm:    LLM::init(&config),
    }
  }

  pub(crate) fn analyze(&mut self) {
    match self.source {
      Some(ref source) => {
        match self.llm.parse(&source.text) {
          Ok(result) => println!("{}",result),
          Err(err)   => log::error!("LLM analysis failed: {}",err),
        }
      }
      None => {},
    }
  }
}
