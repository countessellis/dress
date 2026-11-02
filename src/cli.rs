use crate::config::*;

///////////// CLI

#[derive(Clone)]
pub(crate) struct CLI {
  pub(crate) config: Config,
}

impl CLI {
  pub(crate) fn new(config: &Config) -> CLI {
    CLI {
     config: config.clone(),
    }
  }
}
