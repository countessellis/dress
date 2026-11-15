use crate::config::*;
use crate::llm::*;
use crate::source::*;

///////////// CLI

#[derive(Clone)]
pub(crate) struct CLI {
  pub(crate) source: Option<Source>,
  pub(crate) llm:    LLM,
}

impl CLI {
  pub(crate) fn new(config: &Config) -> Self {
    Self {
     source: Source::load(&config.get("source")),
     llm:    LLM::init(&config),
    }
  }

  pub(crate) fn analyze(&mut self) {
    match self.source {
      Some(ref mut source) => if let Some(text) = source.analyze(&mut self.llm) { println!("{}",text); },
      None => {},
    }
  }
}
