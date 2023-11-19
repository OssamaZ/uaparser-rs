use super::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RegexFile {
  pub user_agent_parsers: Vec<UserAgentParser>,
}

#[derive(Serialize, Deserialize)]
pub struct UserAgentParser {
  pub regex: String,
  pub family_replacement: Option<String>,
  pub v1_replacement: Option<String>,
  pub v2_replacement: Option<String>,
  pub v3_replacement: Option<String>,
}
