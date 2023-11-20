use crate::parser::Parser;
use regex::{Captures, Regex};

macro_rules! capture {
  ($parser:ident, $replacement:ident, $result:expr, $captures:ident, $capture_index:literal) => {
    if let Some(ref $replacement) = $parser.$replacement {
      $result = replace_captures($replacement, &$captures);
    } else {
      $result = $captures
        .get($capture_index)
        .map_or_else(|| "", |m| m.as_str())
        .to_owned();
    }
  };
}

macro_rules! optional_capture {
  ($parser:ident, $replacement:ident, $result:expr, $captures:ident, $capture_index:literal) => {
    if let Some(ref $replacement) = $parser.$replacement {
      $result = Some(replace_captures($replacement, &$captures));
    } else {
      $result = $captures.get($capture_index).and_then(|m| {
        let s = m.as_str();
        if s == "" {
          None
        } else {
          Some(s.to_owned())
        }
      });
    }
  };
}

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
}

impl Default for UserAgent {
  fn default() -> Self {
    UserAgent {
      family: "Other".to_owned(),
      major: None,
      minor: None,
      patch: None,
    }
  }
}

#[derive(Debug)]
pub struct UserAgentMatcher {
  pub regex: Regex,
  pub family_replacement_has_group: bool,
  pub family_replacement: Option<String>,
  pub v1_replacement: Option<String>,
  pub v2_replacement: Option<String>,
  pub v3_replacement: Option<String>,
}

impl Parser for UserAgentMatcher {
  type Item = UserAgent;
  fn parse(&self, str: String) -> Option<Self::Item> {
    if let Some(captures) = self.regex.captures(str.as_str()) {
      let mut user_agent = UserAgent::default();
      capture!(self, family_replacement, user_agent.family, captures, 1);
      optional_capture!(self, v1_replacement, user_agent.major, captures, 2);
      optional_capture!(self, v2_replacement, user_agent.minor, captures, 3);
      optional_capture!(self, v3_replacement, user_agent.patch, captures, 4);
      return Some(user_agent);
    }
    None
  }
}
