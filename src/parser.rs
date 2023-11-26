use crate::{
  device::{Device, DeviceMatcher},
  error::UAParserError,
  user_agent::{UserAgent, UserAgentMatcher},
  yaml_file::YamlFile,
};
use regex::Regex;
use std::fs;

pub trait Parser {
  type Item;
  fn parse(&self, str: String) -> Option<Self::Item>;
}

#[derive(Debug)]
pub struct UAParser {
  ua_matchers: Vec<UserAgentMatcher>,
  device_matchers: Vec<DeviceMatcher>,
}

#[derive(Debug)]
pub struct Client {
  pub user_agent: UserAgent,
  pub device: Device,
}

impl UAParser {
  pub fn from_yaml(path: &str) -> Result<Self, UAParserError> {
    let file = fs::File::open(path)?;
    let regex_file: YamlFile = serde_yaml::from_reader(file)?;
    let mut ua_matchers = Vec::with_capacity(regex_file.ua_parsers.len());
    let mut device_matchers = Vec::with_capacity(regex_file.device_parsers.len());
    for parser in regex_file.ua_parsers {
      ua_matchers.push(UserAgentMatcher {
        regex: Regex::new(&parser.regex)?,
        family_replacement: parser.family_replacement,
        v1_replacement: parser.v1_replacement,
        v2_replacement: parser.v2_replacement,
        v3_replacement: parser.v3_replacement,
        v4_replacement: parser.v4_replacement,
      });
    }
    for parser in regex_file.device_parsers {
      device_matchers.push(DeviceMatcher {
        regex: Regex::new(&parser.regex)?,
        device_replacement: parser.device_replacement,
        brand_replacement: parser.brand_replacement,
        model_replacement: parser.model_replacement,
      });
    }
    Ok(Self {
      ua_matchers,
      device_matchers,
    })
  }

  pub fn parse(&self, user_agent: String) -> Client {
    Client {
      user_agent: self
        .ua_matchers
        .iter()
        .find_map(|matcher| matcher.parse(user_agent.clone()))
        .unwrap_or_default(),
      device: self
        .device_matchers
        .iter()
        .find_map(|matcher| matcher.parse(user_agent.clone()))
        .unwrap_or_default(),
    }
  }
}
