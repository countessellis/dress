use std::{
  env,
};

use crate::defaults::*;
use crate::fields::*;

pub(crate) fn bin_name() -> String {
  match env::current_exe() {
    Ok(path) => {
       let path = path.as_path();
       match path.file_name() {
         Some(file_name) => match file_name.to_str() {
           Some(file_name) => return file_name.to_string(),
           None => {},
         },
         None => {},
       }
    },
    Err(_) => {},
  }
  BUILD_NAME.to_string()
}

pub fn render(template: &str, fields: &Fields) -> String {
  let mut prompt: String = template.to_string();
  for (key,value) in &fields.fields {
    prompt = prompt.replace(&format!("{{{{{}}}}}",key.to_uppercase()),&value.trim());
  }
  prompt
}

// Helper:

pub fn image(name: &str) -> &[u8] {
  match name {
    "/images/dress.png" => DRESS_SMALL_PNG,
    "/images/banner.png" => BANNER_PNG,
    "/images/watermark.png" => WATERMARK_PNG,
    _ => &[],
  }
}

