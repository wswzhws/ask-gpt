use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Debug)]
struct DeepSeekInfo {
    base_url: Url,
}
