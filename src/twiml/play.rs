use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::Digits;

/// TwiML Voice: <Play>
/// https://www.twilio.com/docs/voice/twiml/play
#[derive(Debug, Clone, TypedBuilder, PartialEq, Eq, Serialize, Deserialize)]
pub struct Play {
    #[serde(rename = "@loop")]
    #[builder(default = 1)]
    pub loop_count: u32,
    #[serde(rename = "@digits")]
    #[builder(default, setter(strip_option))]
    pub digits: Option<Digits>,
    #[builder(default, setter(strip_option))]
    #[serde(rename = "#text")]
    pub url: Option<String>,
}
