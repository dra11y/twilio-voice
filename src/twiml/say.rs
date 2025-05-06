use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use super::{Language, Voice, VoicePrice, voices::GENERATIVE_VOICE_PRICE};

/// TwiML Voice: <Say>
/// https://www.twilio.com/docs/voice/twiml/say
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, TypedBuilder, PartialEq, Eq, Serialize, Deserialize)]
pub struct Say {
    #[serde(rename = "$value")]
    pub text: String,
    #[serde(rename = "@language")]
    #[builder(default, setter(strip_option))]
    pub language: Option<Language>,
    #[serde(rename = "@voice")]
    #[builder(default, setter(strip_option))]
    pub voice: Option<Voice>,
    #[serde(rename = "@loop")]
    #[builder(default = 1)]
    pub loop_count: u32,
}

impl VoicePrice for Say {
    fn price(&self) -> Option<f32> {
        self.voice
            .and_then(|v| v.price())
            .map(|p| p * (self.text.len() / 100) as f32)
    }
}
