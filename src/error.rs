use regex::Error as RegexError;
use serde_yaml::Error as YamlError;
use std::io::Error as IOError;

#[derive(Debug)]
pub enum UAParserError {
  Io(IOError),
  Regex(regex::Error),
  Yaml(serde_yaml::Error),
}

impl From<IOError> for UAParserError {
  fn from(value: IOError) -> Self {
    Self::Io(value)
  }
}

impl From<RegexError> for UAParserError {
  fn from(value: RegexError) -> Self {
    Self::Regex(value)
  }
}

impl From<YamlError> for UAParserError {
  fn from(value: YamlError) -> Self {
    Self::Yaml(value)
  }
}
