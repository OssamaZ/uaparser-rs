# User Agent Parser

Simple implementation of the user agent parser based on the [ua-parser/specification](https://github.com/ua-parser/uap-core/blob/master/docs/specification.md).

## Installation

Install the package through Cargo

```
cargo add uaparser-rs
```

Or add manually to your `Cargo.toml` file

```toml
[dependencies]

# Add the dependency
uaparser-rs = "0.1.0"
```

## Usage

```rust
use uaparser_rs::UAParser;

let uap = UAParser::from_yaml("./regexes.yaml").unwrap();
let ua_str = "Mozilla/5.0 (Linux; Android 4.0.1; Galaxy Nexus Build/ITL41F) AppleWebKit 537.31 (KHTML, like Gecko) Chrome/26.0.1410.58 Mobile Safari/537.31";

let client = uap.parse(ua_str);
```

`parse(&str)` returns a client object containing browser, os and device data.

```rust
  client {
    user_agent {
      family: String,
      major: Option<String>,
      minor: Option<String>,
      patch: Option<String>,
      patch_minor: Option<String>,
    },
    os: {
      family: String,
      major: Option<String>,
      minor: Option<String>,
      patch: Option<String>,
      patch_minor: Option<String>,
    },
    device: {
      family: String,
      brand: Option<String>,
      model: Option<String>,
    }
  }

```

Example:

```rust
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

---

TODO:

- [ ] Add an `is_bot() -> bool` for known bot agents.
- [ ] Add a [FromStr](https://doc.rust-lang.org/std/str/trait.FromStr.html) implementation.
- [ ] Use [Clone on Write](https://doc.rust-lang.org/std/borrow/enum.Cow.html) to avoid unnecessary allocations.
