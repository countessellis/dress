use html2md::parse_html;
use std::{
  fs,
  path::Path,
};
use ureq::{
  Agent,
  config::Config,
  tls::{TlsConfig,TlsProvider}
};

///////////// Source

#[derive(Clone)]
pub(crate) struct Source {
  pub(crate) text: String,
}

impl Source {
  pub(crate) fn load(source: &String) -> Option<Source> {
    let text: String = if source.starts_with("http") {
    let tls_config = TlsConfig::builder().provider(TlsProvider::NativeTls).disable_verification(true).build();
    let agent: Agent = Config::builder().tls_config(tls_config).build().into();
      let mut response = match agent.get(source).call() {
        Ok(response) => response,
        Err(err)     => {
          log::error!("Failed to import source from {}: {}",source,err);
          return None;
        },
      };
      let status_code = response.status();
      if status_code.is_success() {
        match response.body_mut().with_config().limit(100000).read_to_vec() {
          Ok(bytes) => {
            match String::from_utf8(bytes.to_vec()) {
              Ok(body) => {
                let text: String = parse_html(&body);
                text.lines().map(str::trim).filter(|line| !line.is_empty()).collect::<Vec<_>>().join("\n")
              },
              Err(err) => {
                log::error!("Failed to parse source from response from {}: {}",source,err);
                return None;
              },
            }
          },
          Err(err) => {
            log::error!("Failed to get source from response from {}: {}",source,err);
            return None;
          },
        }
      } else if status_code == 404 {
        log::warn!("Source not found at {}.",source);
        return None;
      } else {
        log::error!("Failed to import source from {}, received status code {}.",source,status_code);
        return None;
      }
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
