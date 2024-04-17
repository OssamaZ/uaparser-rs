# User Agent Parser

Simple implementation of the user agent parser based on the [ua-parser/specification](https://github.com/ua-parser/uap-core/blob/master/docs/specification.md).

## Usage

```rust
use uaparser_rs::UAParser;

let uap = UAParser::from_yaml("./regexes.yaml").unwrap();
let ua_str = "Mozilla/5.0 (Linux; Android 4.0.1; Galaxy Nexus Build/ITL41F) AppleWebKit 537.31 (KHTML, like Gecko) Chrome/26.0.1410.58 Mobile Safari/537.31";

let client = uap.parse(ua_str);

/*
  client {
    user_agent: {
      family,
      major,
      minor,
      patch,
      patch_minor,
    },
    os: {
      family,
      major,
      minor,
      patch,
      patch_minor,
    },
    device: {
      family,
      brand,
      model,
    }
  }
*/

// User Agent
assert_eq!(client.user_agent.family, "Chrome Mobile");
assert_eq!(client.user_agent.major, Some(String::from("26")));
assert_eq!(client.user_agent.minor, Some(String::from("0")));
assert_eq!(client.user_agent.patch, Some(String::from("1410")));
assert_eq!(client.user_agent.patch_minor, Some(String::from("58")));

// Os
assert_eq!(client.os.family, "Android");
assert_eq!(client.os.major, Some(String::from("4")));
assert_eq!(client.os.minor, Some(String::from("0")));
assert_eq!(client.os.patch, Some(String::from("1")));

// Device
assert_eq!(client.device.family, "Samsung Galaxy Nexus");
assert_eq!(client.device.brand, Some(String::from("Samsung")));
assert_eq!(client.device.model, Some(String::from("Galaxy Nexus")));
```
