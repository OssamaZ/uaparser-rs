//! This is a simple implementation of the user agent parser based on the
//! [ua-parser/specification](https://github.com/ua-parser/uap-core/blob/master/docs/specification.md).
//!
//! ```rust
//! use uaparser_rs::UAParser;
//! let uap = UAParser::from_yaml("./src/regexes.yaml").expect("Unable to load regexes file.");
//! let ua_str = String::from("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36");
//! let result = uap.parse(ua_str);
//! assert_eq!(result.user_agent.family, "Chrome");
//! ```
//! You can also use the string itself
//! ```rust
// ! let uap = UAParser::from_yaml("./src/regexes.yaml").expect("Unable to load regexes file.");
// ! let ua_str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36";
// ! let user_agent: user_agent = ua_str.parse().expect("Unable to parse string");
// ! assert_eq!(user_agent.family, "user_agent");
// ! assert_eq!(user_agent.family, "user_agent");
//! ```
//!

mod error;
mod parser;
mod user_agent;
mod yaml_file;

pub use parser::{Client, UAParser};

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_1() {
    let uap = UAParser::from_yaml("./src/regexes.yaml").unwrap();
    let ua1 = String::from("Mozilla/5.0 (Linux; Android 4.1.1; SPH-L710 Build/JRO03L) AppleWebKit/535.19 (KHTML, like Gecko) Chrome/18.0.1025.166 Mobile Safari/535.19");
    let client = uap.parse(ua1);
    assert_eq!(client.user_agent.family, "Chrome Mobile");
    assert_eq!(client.user_agent.major, Some(String::from("18")));
    assert_eq!(client.user_agent.minor, Some(String::from("0")));
    assert_eq!(client.user_agent.patch, Some(String::from("1025")));
    assert_eq!(client.user_agent.patch_minor, Some(String::from("166")));
  }
}
