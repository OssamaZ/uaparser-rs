//! # User Agent Parser
//! Simple implementation of the user agent parser based on the
//! [ua-parser/specification](https://github.com/ua-parser/uap-core/blob/master/docs/specification.md).
//!
//! ## Installation
//! Install the package through Cargo
//! ```rust,ignore
//! cargo add uaparser-rs
//! ```
//! Or add manually to your `Cargo.toml` file
//!
//! ```rust,ignore
//! [dependencies]
//!
//! # Add the dependency
//! uaparser-rs = "0.1.0"
//! ```
//! ## Usage
//!
//! ```rust,ignore
//! use uaparser_rs::UAParser;
//! let uap = UAParser::from_yaml("./regexes.yaml").unwrap();
//! let ua_str = "Mozilla/5.0 (Linux; Android 4.0.1; Galaxy Nexus Build/ITL41F) AppleWebKit 537.31 (KHTML, like Gecko) Chrome/26.0.1410.58 Mobile Safari/537.31";
//!
//! let client = uap.parser(ua_str);
//! ```
//! `parse(&str)` returns a client object containing browser, os and device data.
//!
//! ```rust,ignore
//! client {
//!   user_agent {
//!     family: String,
//!     major: Option<String>,
//!     minor: Option<String>,
//!     patch: Option<String>,
//!     patch_minor: Option<String>,
//!   },
//!   os: {
//!     family: String,
//!     major: Option<String>,
//!     minor: Option<String>,
//!     patch: Option<String>,
//!     patch_minor: Option<String>,
//!   },
//!   device: {
//!     family: String,
//!     brand: Option<String>,
//!     model: Option<String>,
//!   }
//! }
//! ```
//! Example:
//!
//! ```rust,ignore
//! // User Agent
//! assert_eq!(client.user_agent.family, "Chrome Mobile");
//! assert_eq!(client.user_agent.major, Some(String::from("26")));
//! assert_eq!(client.user_agent.minor, Some(String::from("0")));
//! assert_eq!(client.user_agent.patch, Some(String::from("1410")));
//! assert_eq!(client.user_agent.patch_minor, Some(String::from("58")));
//! // Os
//! assert_eq!(client.os.family, "Android");
//! assert_eq!(client.os.major, Some(String::from("4")));
//! assert_eq!(client.os.minor, Some(String::from("0")));
//! assert_eq!(client.os.patch, Some(String::from("1")));
//! // Device
//! assert_eq!(client.device.family, "Samsung Galaxy Nexus");
//! assert_eq!(client.device.brand, Some(String::from("Samsung")));
//! assert_eq!(client.device.model, Some(String::from("Galaxy Nexus")));
//! ```

mod device;
mod error;
mod os;
mod parser;
mod user_agent;
mod yaml_file;

pub use device::Device;
pub use os::Os;
pub use parser::{Client, UAParser};
pub use user_agent::UserAgent;

#[cfg(test)]
mod test {
  use super::*;
  use serde::Deserialize;
  use std::fs;

  #[test]
  fn ua_tests() {
    #[derive(Deserialize, Debug)]
    struct UATestFile {
      test_cases: Vec<UATest>,
    }

    #[derive(Deserialize, Debug)]
    struct UATest {
      #[serde(rename = "user_agent_string")]
      ua_str: String,
      family: String,
      major: Option<String>,
      minor: Option<String>,
      patch: Option<String>,
      patch_minor: Option<String>,
    }

    let uap = UAParser::from_yaml("./regexes.yaml").unwrap();
    let tests_file =
      fs::File::open("./src/tests/ua_tests.yaml").expect("Unable to open the ua test file.");
    let parsed_tests: UATestFile =
      serde_yaml::from_reader(tests_file).expect("Unable to parse ua test file.");

    let failures: Vec<_> = parsed_tests
      .test_cases
      .iter()
      .map_while(|uat| {
        let Client { user_agent, .. } = uap.parse(&uat.ua_str);
        if user_agent.family == uat.family
          && user_agent.major == uat.major
          && user_agent.minor == uat.minor
          && user_agent.patch == uat.patch
          && user_agent.patch_minor == uat.patch_minor
        {
          return None;
        }
        println!("Expected: {:#?}", uat);
        println!("Got: {:#?}", user_agent);
        println!("--");
        Some(uat.ua_str.as_str())
      })
      .collect();

    assert!(failures.is_empty());
  }

  #[test]
  fn os_tests() {
    #[derive(Deserialize, Debug)]
    struct OsTestFile {
      test_cases: Vec<OsTest>,
    }

    #[derive(Deserialize, Debug)]
    struct OsTest {
      #[serde(rename = "user_agent_string")]
      ua_str: String,
      family: String,
      major: Option<String>,
      minor: Option<String>,
      patch: Option<String>,
      patch_minor: Option<String>,
    }

    let uap = UAParser::from_yaml("./regexes.yaml").unwrap();
    let tests_file =
      fs::File::open("./src/tests/os_tests.yaml").expect("Unable to open the ua test file.");
    let parsed_tests: OsTestFile =
      serde_yaml::from_reader(tests_file).expect("Unable to parse ua test file.");

    let failures: Vec<_> = parsed_tests
      .test_cases
      .iter()
      .map_while(|uat| {
        let Client { os, .. } = uap.parse(&uat.ua_str);
        if os.family == uat.family
          && os.major == uat.major
          && os.minor == uat.minor
          && os.patch == uat.patch
          && os.patch_minor == uat.patch_minor
        {
          return None;
        }
        println!("Expected: {:#?}", uat);
        println!("Got: {:#?}", os);
        println!("--");
        Some(uat.ua_str.as_str())
      })
      .collect();

    assert!(failures.is_empty());
  }

  #[test]
  fn device_tests() {
    #[derive(Deserialize, Debug)]
    struct DeviceTestFile {
      test_cases: Vec<DeviceTest>,
    }

    #[derive(Deserialize, Debug)]
    struct DeviceTest {
      #[serde(rename = "user_agent_string")]
      ua_str: String,
      family: String,
      brand: Option<String>,
      model: Option<String>,
    }

    let uap = UAParser::from_yaml("./regexes.yaml").unwrap();
    let tests_file =
      fs::File::open("./src/tests/device_tests.yaml").expect("Unable to open the ua test file.");
    let parsed_tests: DeviceTestFile =
      serde_yaml::from_reader(tests_file).expect("Unable to parse ua test file.");

    let failures: Vec<_> = parsed_tests
      .test_cases
      .iter()
      .map_while(|uat| {
        let Client { device, .. } = uap.parse(&uat.ua_str);
        if device.family == uat.family && device.brand == uat.brand && device.model == uat.model {
          return None;
        }
        println!("Expected: {:#?}", uat);
        println!("Got: {:#?}", device);
        println!("--");
        Some(uat.ua_str.as_str())
      })
      .collect();

    assert!(failures.is_empty());
  }
}
