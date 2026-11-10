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
  pub(crate) fn new(config: &Config) -> Self {
    Self {
     config: config.clone(),
     source: Source::load(&config.get("source")),
     llm:    LLM::init(&config),
    }
  }

  pub(crate) fn analyze(&mut self) {
    match self.source {
      Some(ref mut source) => source.analyze(&mut self.llm),
      None => {},
    }
  }
}
