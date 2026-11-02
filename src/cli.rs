use crate::config::*;
use crate::source::*;

///////////// CLI

#[derive(Clone)]
pub(crate) struct CLI {
  pub(crate) config: Config,
  pub(crate) source: Option<Source>,
}

impl CLI {
  pub(crate) fn new(config: &Config) -> CLI {
    CLI {
     config: config.clone(),
     source: Source::load(&config.get("source")),
    }
  }

  pub(crate) fn analyze(&self) {
    match self.source {
      Some(ref source) => println!("{}",source.text),
      None => {},
    }
  }
}
