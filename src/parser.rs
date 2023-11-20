use crate::{
  error::UAParserError,
  file::RegexFile,
  user_agent::{UserAgent, UserAgentMatcher},
};
use regex::Regex;
use serde_yaml;
use std::fs;

pub trait Parser {
  type Item;
  fn parse(&self, str: String) -> Option<Self::Item>;
}

#[derive(Debug)]
pub struct UAParser {
  browser_matchers: Vec<UserAgentMatcher>,
}

#[derive(Debug)]
pub struct Client {
  pub user_agent: UserAgent,
}

impl UAParser {
  pub fn init(path: &str) -> Result<Self, UAParserError> {
    let file = fs::File::open(path)?;
    let regex_file: RegexFile = serde_yaml::from_reader(file)?;
    let mut browser_matchers = Vec::with_capacity(regex_file.user_agent_parsers.len());
    for parser in regex_file.user_agent_parsers {
      browser_matchers.push(UserAgentMatcher {
        regex: Regex::new(&parser.regex)?,
        family_replacement_has_group: parser
          .family_replacement
          .as_ref()
          .map_or(false, |x| x.as_str().contains('$')),
        family_replacement: parser.family_replacement,
        v1_replacement: parser.v1_replacement,
        v2_replacement: parser.v2_replacement,
        v3_replacement: parser.v3_replacement,
      });
    }
    Ok(Self { browser_matchers })
  }

  pub fn parse(&self, user_agent: String) -> Client {
    Client {
      user_agent: self._parse_user_agent(user_agent),
    }
  }

  fn _parse_user_agent(&self, user_agent: String) -> UserAgent {
    self
      .browser_matchers
      .iter()
      .find_map(|matcher| matcher.parse(user_agent.clone()))
      .unwrap_or_default()
  }
}
