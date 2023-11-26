use crate::parser::Parser;
use regex::{Captures, Regex};

fn replace_captures(string: &str, captures: &Captures) -> String {
  let mut result = string.to_owned();
  for i in 1..captures.len() {
    result = result.replace(
      &format!("${}", i),
      captures.get(i).map_or_else(|| "", |m| m.as_str()),
    );
  }
  result
}

#[derive(Debug)]
pub struct Device {
  pub family: String,
  pub brand: Option<String>,
  pub model: Option<String>,
}

impl Default for Device {
  fn default() -> Self {
    Device {
      family: "Other".to_owned(),
      brand: None,
      model: None,
    }
  }
}

#[derive(Debug)]
pub struct DeviceMatcher {
  pub regex: Regex,
  pub device_replacement: Option<String>,
  pub brand_replacement: Option<String>,
  pub model_replacement: Option<String>,
}

impl Parser for DeviceMatcher {
  type Item = Device;
  fn parse(&self, str: String) -> Option<Self::Item> {
    if let Some(captures) = self.regex.captures(str.as_str()) {
      let mut device = Device::default();
      // name
      device.family = match &self.device_replacement {
        Some(device_replacement) => replace_captures(device_replacement, &captures),
        None => captures
          .get(1)
          .map_or_else(|| "", |m| m.as_str())
          .to_owned(),
      };

      // brand
      device.brand = match &self.brand_replacement {
        Some(brand_replacement) => Some(replace_captures(brand_replacement, &captures)),
        None => captures.get(2).and_then(|m| {
          let s = m.as_str();
          if s == "" {
            None
          } else {
            Some(s.to_owned())
          }
        }),
      };

      // model
      device.model = match &self.model_replacement {
        Some(model_replacement) => Some(replace_captures(model_replacement, &captures)),
        None => captures.get(3).and_then(|m| {
          let s = m.as_str();
          if s == "" {
            None
          } else {
            Some(s.to_owned())
          }
        }),
      };

      return Some(device);
    }
    None
  }
}
