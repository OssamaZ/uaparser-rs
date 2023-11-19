//! This is a simple implementation of the user agent parser based on the
//! [ua-parser/specification](https://github.com/ua-parser/uap-core/blob/master/docs/specification.md).
//!
//! ```rust
//! use uaparser_rs::UAParser;
//! let uap = UAParser::init("./src/regexes.yaml").expect("Unable to load regexes file.");
//! let ua_str = String::from("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36");
//! let result = uap.parse(ua_str);
//! assert_eq!(result.user_agent.family, "Chrome");
//! ```
//! You can also use the string itself
//! ```rust
// ! let uap = UAParser::init("./src/regexes.yaml").expect("Unable to load regexes file.");
// ! let ua_str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36";
// ! let user_agent: user_agent = ua_str.parse().expect("Unable to parse string");
// ! assert_eq!(user_agent.family, "user_agent");
// ! assert_eq!(user_agent.family, "user_agent");
//! ```
//!

use error::UAParserError;
use file::RegexFile;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use user_agent::{UserAgent, UserAgentMatcher};

mod error;
mod file;
mod user_agent;

pub trait Parser {
  type Item;
  fn parse(&self, str: String) -> Option<Self::Item>;
}

#[derive(Debug)]
pub struct Client {
  pub user_agent: UserAgent,
}

#[derive(Debug)]
pub struct UAParser {
  browser_matchers: Vec<UserAgentMatcher>,
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

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn it_works() {
    let ua1 = String::from("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36");
    let uap = UAParser::init("./src/regexes.yaml").unwrap();
    let res = uap.parse(ua1);
    println!("{res:?}");
    assert_eq!(res.user_agent.family, "Chrome");
  }
}
