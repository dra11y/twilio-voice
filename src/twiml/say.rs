use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use super::TwilioLanguage;

/// TwiML Voice: <Say>
/// https://www.twilio.com/docs/voice/twiml/say
#[derive(Debug, Clone, TypedBuilder, PartialEq, Eq, Serialize, Deserialize)]
pub struct Say {
    #[serde(rename = "$value")]
    pub text: String,
    #[serde(rename = "@language")]
    #[builder(default)]
    pub language: TwilioLanguage,
    #[serde(rename = "@voice")]
    pub voice: super::voices::Voice,
    #[serde(rename = "@loop")]
    #[builder(default = 1)]
    pub loop_count: u32,
}
