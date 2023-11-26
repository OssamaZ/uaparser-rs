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
pub struct UserAgent {
  pub family: String,
  pub major: Option<String>,
  pub minor: Option<String>,
  pub patch: Option<String>,
  pub patch_minor: Option<String>,
}

impl Default for UserAgent {
  fn default() -> Self {
    UserAgent {
      family: "Other".to_owned(),
      major: None,
      minor: None,
      patch: None,
      patch_minor: None,
    }
  }
}

#[derive(Debug)]
pub struct UserAgentMatcher {
  pub regex: Regex,
  pub family_replacement: Option<String>,
  pub v1_replacement: Option<String>,
  pub v2_replacement: Option<String>,
  pub v3_replacement: Option<String>,
  pub v4_replacement: Option<String>,
}

impl Parser for UserAgentMatcher {
  type Item = UserAgent;
  fn parse(&self, str: String) -> Option<Self::Item> {
    if let Some(captures) = self.regex.captures(str.as_str()) {
      let mut user_agent = UserAgent::default();
      // family
      user_agent.family = match &self.family_replacement {
        Some(family_replacement) => replace_captures(family_replacement, &captures),
        None => captures
          .get(1)
          .map_or_else(|| "", |m| m.as_str())
          .to_owned(),
      };

      // major
      user_agent.major = match &self.v1_replacement {
        Some(v1_replacement) => Some(replace_captures(v1_replacement, &captures)),
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
      user_agent.minor = match &self.v2_replacement {
        Some(v2_replacement) => Some(replace_captures(v2_replacement, &captures)),
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
      user_agent.patch = match &self.v3_replacement {
        Some(v3_replacement) => Some(replace_captures(v3_replacement, &captures)),
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
      user_agent.patch_minor = match &self.v4_replacement {
        Some(v4_replacement) => Some(replace_captures(v4_replacement, &captures)),
        None => captures.get(5).and_then(|m| {
          let s = m.as_str();
          if s == "" {
            None
          } else {
            Some(s.to_owned())
          }
        }),
      };

      return Some(user_agent);
    }
    None
  }
}
