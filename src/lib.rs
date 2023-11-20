//! This is a simple implementation of the user agent parser based on the
//! [ua-parser/specification](https://github.com/ua-parser/uap-core/blob/master/docs/specification.md).
//!
//! ```rust
//! use uaparser_rs::parser::UAParser;
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

mod error;
mod file;
pub mod parser;
mod user_agent;

#[cfg(test)]
mod test {
  use super::parser::UAParser;

  #[test]
  fn it_works() {
    let ua1 = String::from("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36");
    let uap = UAParser::init("./src/regexes.yaml").unwrap();
    let res = uap.parse(ua1);
    println!("{res:?}");
    assert_eq!(res.user_agent.family, "Chrome");
  }
}
