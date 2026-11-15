use html2md::rewrite_html;
use regex::Regex;
use std::{
  fs,
  path::Path,
  sync::OnceLock,
};
use ureq::{
  Agent,
  config::Config,
  tls::{TlsConfig,TlsProvider}
};

use crate::llm::*;

///////////// Source

#[derive(Clone)]
pub(crate) struct Source {
  pub(crate) text: String,
}

impl Source {
  pub(crate) fn load(source: &String) -> Option<Source> {
    let text: String = if source.starts_with("http") {
      log::info!("Source provided appears to be a web address: {}",source);
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
        match response.body_mut().with_config().limit(5000000).read_to_vec() {
          Ok(bytes) => {
            match String::from_utf8(bytes) {
              Ok(body) => {
                let content_type: Option<&str> = response.headers().get("content-type").and_then(|content_type| content_type.to_str().ok());
                if content_type.map_or(false, |ct| ct.contains("text/html")) {
                  Self::convert_html(&body)
                } else {
                  if content_type.is_none() {
                    log::warn!("No Content-Type header present in the response.");
                  }
                  body
                }
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
      log::info!("Source provided appears to be a local path: {}",source);
      let path = Path::new(source);
      match fs::read_to_string(source) {
        Ok(text) => {
          log::info!("Loaded source from {}.",source);
          let extension = path.extension().and_then(|ext| ext.to_str()).map(|ext| ext.to_lowercase());
          match extension.as_deref() {
            Some("html")|Some("htm") => {
              Self::convert_html(&text)
            },
            _ => text,
          }
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

  fn convert_html(text: &String) -> String {
    static JSON_LD_REGEX: OnceLock<Regex> = OnceLock::new();
    let text = text.clone();
    let script_regex = JSON_LD_REGEX.get_or_init(|| {
      Regex::new(r#"(?is)<script[^>]*type=["']application/ld\+json["'][^>]*>(.*?)</script>"#).unwrap()
    });
    for cap in script_regex.captures_iter(&text) {
      let text: &str = cap[1].trim();
      if text.contains("\"@type\"") && (text.contains("\"Recipe\"") || text.contains("\"HowTo\"")) {
        let markdown_text: String = rewrite_html(&text,false);
        return markdown_text.lines().map(str::trim).filter(|line| !line.is_empty()).collect::<Vec<_>>().join("\n");
      }
    }
    static SCRIPT_REMOVE: OnceLock<Regex> = OnceLock::new();
    static STYLE_REMOVE: OnceLock<Regex> = OnceLock::new();
    let no_scripts = SCRIPT_REMOVE.get_or_init(|| Regex::new(r#"(?is)<script[^>]*>.*?</script>"#).unwrap()).replace_all(&text, "");
    let text = STYLE_REMOVE.get_or_init(|| Regex::new(r#"(?is)<style[^>]*>.*?</style>"#).unwrap()).replace_all(&no_scripts, "");
    let markdown_text: String = rewrite_html(&text,false);
    markdown_text.lines().map(str::trim).filter(|line| !line.is_empty()).collect::<Vec<_>>().join("\n")
  }

  pub(crate) fn analyze(&mut self, llm: &mut LLM) -> Option<String>{
    match llm.parse(&self.text) {
      Ok(result) => Some(format!("{}",result)),
      Err(err)   => {
        log::error!("LLM analysis failed: {}",err);
        None
      },
    }
  }
}
