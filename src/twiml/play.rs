use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use super::Digits;

/// TwiML Voice: <Play>
/// https://www.twilio.com/docs/voice/twiml/play
#[derive(Debug, Clone, TypedBuilder, PartialEq, Eq, Serialize, Deserialize)]
pub struct Play {
    #[serde(rename = "$value")]
    pub url: String,
    #[serde(rename = "@loop")]
    #[builder(default = 1)]
    pub loop_count: u32,
    #[serde(rename = "@digits")]
    #[builder(default)]
    pub digits: Option<Digits>,
}
