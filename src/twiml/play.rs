use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use super::Digits;

/// TwiML Voice: <Play>
/// https://www.twilio.com/docs/voice/twiml/play
#[derive(Debug, Clone, TypedBuilder, PartialEq, Eq, Serialize, Deserialize)]
pub struct Play {
    #[serde(rename = "@loop")]
    #[builder(default = 1)]
    pub loop_count: u32,
    #[serde(rename = "@digits")]
    #[builder(default)]
    pub digits: Option<Digits>,
    #[serde(rename = "#text")]
    pub url: String,
}

impl From<&str> for Play {
    fn from(value: &str) -> Self {
        Play::builder().url(value.into()).build()
    }
}

impl From<String> for Play {
    fn from(value: String) -> Self {
        Play::from(value.as_str())
    }
}
