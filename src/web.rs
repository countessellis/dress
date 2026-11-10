use crossbeam_channel::{unbounded,Receiver,Sender};
use std::{
  collections::HashMap,
  str::FromStr,
  thread,
  time::Duration,
};
use tiny_http::{Header,Request,Response};
use url::Url;

use crate::util;

use crate::config::*;
use crate::defaults::*;
use crate::fields::*;
use crate::llm::*;
use crate::source::*;

///////////// Web

#[derive(Debug, Clone)]
pub struct Web {
  config: Config,
  llm:    LLM,
}

impl Web {
  pub(crate) fn new(config: &Config) -> Self {
    Self {
     config: config.clone(),
     llm:    LLM::init(&config),
    }
  }

  pub(crate) fn analyze(&mut self, source: &mut Source) {
    source.analyze(&mut self.llm);
  }

  pub fn listen(&mut self) {
    let (incoming_tx,incoming_rx) = unbounded::<Request>();
    let server = match tiny_http::Server::http(self.config.get("listen").clone()) {
      Ok(server) => server,
      Err(err) => {
        log::error!("Failed to initialize listener: {}",err);
        return;
      },
    };
    log::info!("Listening on {}.",self.config.get("listen"));
    thread::spawn(move || {
      for request in server.incoming_requests() {
        if incoming_tx.send(request).is_err() {
          log::error!("Server handler disconnected. Stopping listener.");
          break;
        }
      }
    });
    thread::spawn({
      let incoming       = incoming_rx.clone();
      move || {
        Self::process(&incoming);
      }
    });
    loop {
      thread::sleep(Duration::from_secs(DEFAULT_DELAY));
    }
  }

  pub(crate) fn process(incoming: &Receiver<Request>) {
    loop {
      for mut request in incoming.try_iter() {
        if let Some(remote) = request.remote_addr() {
          log::debug!("Received connection from {}:{}",remote.ip(),remote.port());
          let base_url = Url::parse("http://localhost").unwrap();
          if let Ok(parsed_url) = base_url.join(request.url()) {
            let path = parsed_url.path().to_string();
            log::info!("Request received for on {}",path);
            thread::spawn({
              let remote = remote.clone();
              move || {
                let mut response = match (request.method().as_str(), path.as_str()) {
                  ("GET","")|("GET","/")|("GET","/index.html") => {
                    let mut fields: Fields = Fields::new(&HashMap::from([
                      ("MENU",format!(
"
<HR>
Home
<HR>
"
                      )),
                      ("BANNER","<IMG SRC=/images/banner.png ALT=Dress>".to_string()),
                      ("TITLE","Dress".to_string()),
                      ("SUBTITLE","".to_string()),
                      ("COPYRIGHT",format!(
"
<HR>
Copyright &copy; {} {}
<HR>
"
                      ,COPYRIGHT,AUTHORS)),
                    ]));
                    fields.insert("TEXT_ENTRY",util::render(TEXT_ENTRY_TEMPLATE,&fields));
                    fields.insert("BODY",util::render(TABS_TEMPLATE,&fields));
                    let body: String = util::render(INDEX_TEMPLATE,&fields);
                    let mut response = Response::from_string(body);
                    if let Ok(content_type_header) = Header::from_str("Content-Type: text/html; charset=utf-8") {
                      response = response.with_header(content_type_header);
                    } else {
                      log::error!("Failed to parse content type header, using default.");
                    }
                    response
                  },
                  ("GET","/default.css") => {
                    let fields: Fields = Fields::new(&HashMap::from([
                      ("WATERMARK",format!("/images/watermark.png"))
                    ]));
                    let body: String = util::render(CSS_TEMPLATE,&fields);
                    let mut response = Response::from_string(body);
                    if let Ok(content_type_header) = Header::from_str("Content-Type: text/css; charset=utf-8") {
                      response = response.with_header(content_type_header);
                    } else {
                      log::error!("Failed to parse content type header, using default.");
                    }
                    response
                  },
                  ("GET","/favicon.ico") => {
                     let mut response = Response::from_data(FAVICON_ICO);
                     if let Ok(content_type_header) = Header::from_str("Content-Type: image/x-icon") {
                       response = response.with_header(content_type_header);
                     } else {
                       log::error!("Failed to parse content type header, using default.");
                     }
                     response
                  },
                  ("GET",path) if path.starts_with("/images/") => {
                    let image: &[u8] = util::image(path);
                    if image.len() == 0 {
                      Response::from_string("Not Found").with_status_code(404)
                    } else {
                      let mut response = Response::from_data(image);
                      if let Ok(content_type_header) = Header::from_str("Content-Type: image/png") {
                        response = response.with_header(content_type_header);
                      } else {
                        log::error!("Failed to parse content type header, using default.");
                      }
                      response
                    }
                  },
                  _ => {
                    Response::from_string("Not Found").with_status_code(404)
                  },
                };
                if let Ok(server_header) = Header::from_str(&format!("Server: {}/{}",APP_NAME,VERSION_ID)) {
                  response = response.with_header(server_header);
                } else {
                  log::error!("Failed to parse server header, using default.");
                }
                match request.respond(response) {
                  Ok(_) => {},
                  Err(err) => match err.kind() {
                    std::io::ErrorKind::BrokenPipe => log::warn!("Client disconnected prematurely (Broken Pipe)."),
                    std::io::ErrorKind::ConnectionReset => log::warn!("Connection was reset by the remote peer."),
                    _ => log::error!("Failed to send response due to an I/O error: {}", err),
                  } 
                }
              }
            });
          }
        }
      }
    }
  }
}
