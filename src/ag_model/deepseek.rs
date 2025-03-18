use std::{fmt, ops::Deref};

use serde::{Deserialize, Serialize};
use url::Url;

struct DeepSeekModel {
    base_url: Url,
    api_key: ApiKey,
}

impl Default for DeepSeekModel {
    fn default() -> Self {
        Self {
            base_url: Url::parse("https://api.deepseek.com/v1").unwrap(),
            api_key: ApiKey::new(""),
        }
    }
}

impl DeepSeekModel {
    pub fn new(api_key: ApiKey) -> Self {
        Self {
            api_key,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone)]
pub struct ApiKey(String);

impl ApiKey {
    pub fn new(key: impl Into<String>) -> Self {
        Self(key.into())
    }
}

impl Deref for ApiKey {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for ApiKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq<&str> for ApiKey {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl PartialEq<String> for ApiKey {
    fn eq(&self, other: &String) -> bool {
        &self.0 == other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_key() {
        let key = ApiKey::new("test");
        assert_eq!(key, "test");
        assert_eq!(key, String::from("test"));
        assert_eq!(key.len(), 4);
        assert_eq!(key.to_uppercase(), "TEST");
    }

    #[test]
    fn test_deepseek_info() {
        let key = ApiKey::new("test_api_key");
        let model = DeepSeekModel::new(key);
        assert_eq!(model.api_key, "test_api_key");
    }

    #[test]
    fn test_deepseek_info_default() {
        let model = DeepSeekModel::default();
        assert_eq!(model.api_key, "");
    }
}
