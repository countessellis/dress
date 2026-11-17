use serde::{Deserialize,Serialize};
use std::{
  collections::HashMap,
  io::{BufRead,BufReader,Read},
  time::Duration,
};

use crate::util;

use crate::collections::Fields;

use crate::config::*;
use crate::defaults::*;

///////////// LLM

#[derive(Debug,Clone)]
pub(crate) struct LLM {
  agent:  ureq::Agent,
  config: Config,
}

#[derive(Serialize)]
pub(crate) struct LLMRequest {
  pub(crate) model:      String,
  pub(crate) prompt:     String,
  pub(crate) system:     String,
  pub(crate) stream:     bool,
  pub(crate) keep_alive: String,
  pub(crate) format:     serde_json::Value,
  pub(crate) options:    Option<OllamaOptions>,
}

#[derive(Deserialize)]
pub(crate) struct LLMResponse {
  pub(crate) response:          String,
  pub(crate) done:              bool,
  pub(crate) total_duration:    Option<u64>,
  pub(crate) eval_duration:     Option<u64>,
  pub(crate) prompt_eval_count: Option<u64>,
  pub(crate) eval_count:        Option<u64>,
}

#[derive(Serialize)]
pub(crate) struct OllamaOptions {
  pub(crate) temperature: f32,
  pub(crate) stop:        Vec<String>,
  pub(crate) num_ctx:     u32,
  pub(crate) num_predict: i32,
}

impl LLM {
  pub(crate) fn init(config: &Config) -> LLM {
    let agent_config = ureq::config::Config::builder().http_status_as_error(false).timeout_connect(Some(Duration::from_secs(10))).timeout_global(Some(Duration::from_secs(2700))).build();
    let agent = ureq::Agent::new_with_config(agent_config);
    LLM {
      agent:  agent,
      config: config.clone(),
    }
  }

  pub(crate) fn parse(&mut self, input: &String) -> Result<String,String> {
    let fields: Fields = Fields::new(&HashMap::from([
      ("TEXT",input.clone()),
    ]));
    let system_prompt: String = util::render(SYSTEM_PROMPT_TEMPLATE,&fields);
    let user_prompt: String = util::render(USER_PROMPT_TEMPLATE,&fields);
    let req: LLMRequest = LLMRequest {
      model:      self.config.get("model"),
      system:     system_prompt,
      prompt:     user_prompt,
      stream:     true,
      keep_alive: "1h".to_string(),
      format:     serde_json::Value::Null,
      options:     Some(OllamaOptions { temperature: Self::get_temperature(&self.config.get("model")).unwrap_or(0.0), stop: Vec::new(), num_ctx: 8192, num_predict: 2048 }),
    };
    log::debug!("Sending text to {} to analyze.",req.model);
    match self.submit(&self.config.get("llm_server"),&req) {
      Ok(resp) => Ok(resp),
      Err(err) => Err(format!("Failed to get result from model: {}",err)),
    }
  }

  pub(crate) fn get_temperature(model_name: &String) -> Option<f32> {
    let model_name: String = model_name.to_lowercase();
    if model_name.contains("sol") || model_name.contains("terra") || model_name.starts_with("o1") || model_name.starts_with("o3") || (model_name.contains("gpt-5.") && !model_name.contains("instant")) || model_name.contains("gpt-6.") {
      None
    } else if model_name.contains("phi4") || model_name.contains("judge") {
      Some(0.1)
    } else if model_name.contains("gemma") || model_name.contains("-mini") || model_name.contains("-coder") || model_name.contains("codellama") {
      Some(0.2)
    } else if model_name.contains("r1") || model_name.contains("think") {
      Some(0.6)
    } else if model_name.starts_with("gpt-4o") || model_name.contains("luna") {
      Some(0.4)
    } else if model_name.contains("mistral") || model_name.contains("ministral") {
      Some(0.7)
    } else {
      Some(0.7)
    }
  }

  pub(crate) fn submit(&mut self, server: &String, req: &LLMRequest) -> Result<String,String> {
    match self.agent.post(server).send_json(&req) {
      Ok(mut resp) => {
        if resp.status().is_client_error() || resp.status().is_server_error() {
          let mut err_body: String = String::new();
          match resp.body_mut().as_reader().read_to_string(&mut err_body) {
            Ok(_)   => return Err(format!("Status Code: {}, Response from {}: {}",resp.status(),req.model,err_body)),
            Err(err) => return Err(format!("Status Code: {}, Failed to get body from {}: {}",resp.status(),req.model,err)),
          }
        } else {
          let reader = BufReader::new(resp.body_mut().as_reader());
          let mut full_response = String::new();
          for line_result in reader.lines() {
            match line_result {
              Ok(line) => {
                if line.trim().is_empty() { continue; }
                match serde_json::from_str::<LLMResponse>(&line) {
                  Ok(chunk) => {
                    full_response.push_str(&chunk.response);
                    if chunk.done {
                      let total_sec = chunk.total_duration.unwrap_or(0) as f64 / 1_000_000_000.0;
                      let eval_sec = chunk.eval_duration.unwrap_or(0) as f64 / 1_000_000_000.0;
                      let prompt_tokens = chunk.prompt_eval_count.unwrap_or(0);
                      let output_tokens = chunk.eval_count.unwrap_or(0);
                      let tokens_per_sec = if eval_sec > 0.0 { output_tokens as f64 / eval_sec } else { 0.0 };
                      log::info!("Metrics -> Prompt Tokens: {} | Output Tokens: {} | Speed: {:.2} t/s | Gen Time: {:.2}s | Total Time: {:.2}s",prompt_tokens,output_tokens,tokens_per_sec,eval_sec, total_sec);
                    }
                  },
                  Err(err) => return Err(format!("Failed to parse stream chunk from {}: {}", req.model, err)),
                }
              },
              Err(err) => return Err(format!("Error reading stream from {}: {}", req.model, err)),
            }
          }
          Ok(full_response)
        }
      },
      Err(err) => return Err(format!("Request to {} failed: {}",req.model,err)),
    }
  }

}
