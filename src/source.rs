use std::fs;
use std::path::Path;

///////////// Source

#[derive(Clone)]
pub(crate) struct Source {
  pub(crate) text: String,
}

impl Source {
  pub(crate) fn load(source: &String) -> Option<Source> {
    let text: String = if source.starts_with("http") {
      return None;
    } else if Path::new(source).is_file() {
      match fs::read_to_string(source) {
        Ok(text) => {
          log::info!("Loaded source from {}.",source);
          text
        },
        Err(err) => {
          log::error!("Failed to import source from {}: {}",source,err);
          return None;
        },
      }
    } else { return None };
    Some(Source {
     text: text,
    })
  }
}
