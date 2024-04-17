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
pub struct Os {
  pub family: String,
  pub major: Option<String>,
  pub minor: Option<String>,
  pub patch: Option<String>,
  pub patch_minor: Option<String>,
}

impl Default for Os {
  fn default() -> Self {
    Os {
      family: "Other".to_owned(),
      major: None,
      minor: None,
      patch: None,
      patch_minor: None,
    }
  }
}

#[derive(Debug)]
pub struct OsMatcher {
  pub regex: Regex,
  pub os_replacement: Option<String>,
  pub os_v1_replacement: Option<String>,
  pub os_v2_replacement: Option<String>,
  pub os_v3_replacement: Option<String>,
  pub os_v4_replacement: Option<String>,
}

impl Parser for OsMatcher {
  type Item = Os;
  fn parse(&self, str: &str) -> Option<Self::Item> {
    if let Some(captures) = self.regex.captures(str) {
      let mut os = Os::default();
      // family
      os.family = match &self.os_replacement {
        Some(os_replacement) => replace_captures(os_replacement, &captures),
        None => captures
          .get(1)
          .map_or_else(|| "", |m| m.as_str())
          .to_owned(),
      };

      // major
      os.major = match &self.os_v1_replacement {
        Some(os_v1_replacement) => Some(replace_captures(os_v1_replacement, &captures)),
        None => captures.get(2).and_then(|m| {
          let s = m.as_str();
          if s == "" {
            None
          } else {
            Some(s.to_owned())
          }
        }),
      };

      // minor
      os.minor = match &self.os_v2_replacement {
        Some(os_v2_replacement) => Some(replace_captures(os_v2_replacement, &captures)),
        None => captures.get(3).and_then(|m| {
          let s = m.as_str();
          if s == "" {
            None
          } else {
            Some(s.to_owned())
          }
        }),
      };

      // patch
      os.patch = match &self.os_v3_replacement {
        Some(os_v3_replacement) => Some(replace_captures(os_v3_replacement, &captures)),
        None => captures.get(4).and_then(|m| {
          let s = m.as_str();
          if s == "" {
            None
          } else {
            Some(s.to_owned())
          }
        }),
      };

      // patch_minor
      os.patch_minor = match &self.os_v4_replacement {
        Some(os_v4_replacement) => Some(replace_captures(os_v4_replacement, &captures)),
        None => captures.get(5).and_then(|m| {
          let s = m.as_str();
          if s == "" {
            None
          } else {
            Some(s.to_owned())
          }
        }),
      };

      return Some(os);
    }
    None
  }
}
