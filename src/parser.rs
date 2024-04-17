use crate::{
  device::DeviceMatcher, error::UAParserError, os::OsMatcher, user_agent::UserAgentMatcher,
  yaml_file::YamlFile, Device, Os, UserAgent,
};
use regex::Regex;
use std::fs;

pub trait Parser {
  type Item;
  fn parse(&self, str: &str) -> Option<Self::Item>;
}

#[derive(Debug)]
pub struct UAParser {
  ua_matchers: Vec<UserAgentMatcher>,
  device_matchers: Vec<DeviceMatcher>,
  os_matchers: Vec<OsMatcher>,
}

#[derive(Debug)]
pub struct Client {
  pub user_agent: UserAgent,
  pub device: Device,
  pub os: Os,
}

impl UAParser {
  pub fn from_yaml(path: &str) -> Result<Self, UAParserError> {
    let file = fs::File::open(path)?;
    let regex_file: YamlFile = serde_yaml::from_reader(file)?;
    let mut ua_matchers = Vec::with_capacity(regex_file.ua_parsers.len());
    let mut device_matchers = Vec::with_capacity(regex_file.device_parsers.len());
    let mut os_matchers = Vec::with_capacity(regex_file.os_parsers.len());
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
    for parser in regex_file.os_parsers {
      os_matchers.push(OsMatcher {
        regex: Regex::new(&parser.regex)?,
        os_replacement: parser.os_replacement,
        os_v1_replacement: parser.os_v1_replacement,
        os_v2_replacement: parser.os_v2_replacement,
        os_v3_replacement: parser.os_v3_replacement,
        os_v4_replacement: parser.os_v4_replacement,
      });
    }
    Ok(Self {
      ua_matchers,
      device_matchers,
      os_matchers,
    })
  }

  pub fn parse(&self, user_agent: &str) -> Client {
    Client {
      user_agent: self
        .ua_matchers
        .iter()
        .find_map(|matcher| matcher.parse(user_agent))
        .unwrap_or_default(),
      os: self
        .os_matchers
        .iter()
        .find_map(|matcher| matcher.parse(user_agent))
        .unwrap_or_default(),
      device: self
        .device_matchers
        .iter()
        .find_map(|matcher| matcher.parse(user_agent))
        .unwrap_or_default(),
    }
  }
}
