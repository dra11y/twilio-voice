use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use super::{KeypadDigit, Pause, Say, TwilioLanguage, TwilioMethod};

/// TwiML Voice: <Gather>
/// https://www.twilio.com/docs/voice/twiml/gather
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, TypedBuilder, PartialEq, Eq, Serialize, Deserialize)]
pub struct Gather {
    #[serde(rename = "@action")]
    #[builder(default)]
    action: String,
    #[serde(rename = "@actionOnEmptyResult")]
    #[builder(default)]
    action_on_empty_result: bool,
    #[serde(rename = "@finishOnKey")]
    #[builder(default)]
    finish_on_key: Option<KeypadDigit>,
    #[serde(rename = "@hints")]
    #[builder(default, setter(strip_option))]
    hints: Option<String>,
    #[serde(rename = "@input")]
    #[builder(default)]
    input: InputType,
    #[serde(rename = "@language")]
    #[builder(default)]
    language: TwilioLanguage,
    #[serde(rename = "@method")]
    #[builder(default)]
    method: TwilioMethod,
    #[serde(rename = "@numDigits")]
    #[builder(default, setter(strip_option))]
    num_digits: Option<u8>,
    #[serde(rename = "@partialResultCallback")]
    #[builder(default, setter(strip_option))]
    partial_result_callback: Option<String>,
    #[serde(rename = "@partialResultCallbackMethod")]
    #[builder(default, setter(strip_option))]
    partial_result_callback_method: Option<TwilioMethod>,
    #[serde(rename = "@profanityFilter")]
    #[builder(default, setter(strip_option))]
    profanity_filter: Option<bool>,
    #[serde(rename = "@speechModel")]
    #[builder(default, setter(strip_option))]
    speech_model: Option<SpeechModel>,
    #[serde(rename = "@speechTimeout")]
    #[builder(default, setter(strip_option))]
    speech_timeout: Option<Timeout>,
    #[serde(rename = "@timeout")]
    #[builder(default, setter(strip_option))]
    timeout: Option<Timeout>,

    #[builder(default)]
    #[serde(default, rename = "$value")]
    verbs: Vec<GatherVerb>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InputType {
    #[default]
    Dtmf,
    Speech,
    DtmfSpeech,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Timeout {
    Auto,
    Seconds(u8),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GatherVerb {
    Say(Say),
    Pause(Pause),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpeechModel {
    Default,
    NumbersAndCommands,
    PhoneCall,
    ExperimentalConversations,
    ExperimentalUtterances,
    Googlev2Long,
    Googlev2Short,
    Googlev2Telephony,
    Googlev2TelephonyShort,
    #[serde(rename = "deepgram_nova-2")]
    DeepgramNova2,
}

impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N>
    GatherBuilder<(A, B, C, D, E, F, G, H, I, J, K, L, M, N, ())>
{
    #[allow(clippy::type_complexity)]
    pub fn say(
        self,
        say: Say,
    ) -> GatherBuilder<(A, B, C, D, E, F, G, H, I, J, K, L, M, N, (Vec<GatherVerb>,))> {
        self.verbs(vec![GatherVerb::Say(say)])
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N>
    GatherBuilder<(A, B, C, D, E, F, G, H, I, J, K, L, M, N, (Vec<GatherVerb>,))>
{
    pub fn say(mut self, say: Say) -> Self {
        self.fields.14.0.push(GatherVerb::Say(say));
        self
    }
}
