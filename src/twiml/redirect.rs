use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use super::Method;

/// TwiML Voice: <Redirect>
/// https://www.twilio.com/docs/voice/twiml/redirect
#[derive(Debug, Clone, TypedBuilder, PartialEq, Eq, Serialize, Deserialize)]
pub struct Redirect {
    #[serde(rename = "$text")]
    pub url: String,
    #[serde(rename = "@method")]
    #[builder(default)]
    pub method: Method,
}
