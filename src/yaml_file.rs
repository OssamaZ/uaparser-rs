use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct YamlFile {
  #[serde(rename = "user_agent_parsers")]
  pub ua_parsers: Vec<UserAgentParser>,
  pub os_parsers: Vec<OsParser>,
  pub device_parsers: Vec<DeviceParser>,
}

#[derive(Serialize, Deserialize)]
pub struct UserAgentParser {
  pub regex: String,
  pub family_replacement: Option<String>,
  pub v1_replacement: Option<String>,
  pub v2_replacement: Option<String>,
  pub v3_replacement: Option<String>,
  pub v4_replacement: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct OsParser {
  pub regex: String,
  pub os_replacement: Option<String>,
  pub os_v1_replacement: Option<String>,
  pub os_v2_replacement: Option<String>,
  pub os_v3_replacement: Option<String>,
  pub os_v4_replacement: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct DeviceParser {
  pub regex: String,
  pub device_replacement: Option<String>,
  pub brand_replacement: Option<String>,
  pub model_replacement: Option<String>,
}
